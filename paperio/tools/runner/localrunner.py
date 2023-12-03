#!/usr/bin/env python3
from asyncio import events
import asyncio
import argparse
import os
import sys

from clients import KeyboardClient, SimplePythonClient, FileClient, TcpClient
from constants import CONSTS
from game_objects.game import LocalGame, Game

TCP_PORT_NUMBER = 8000

loop = events.new_event_loop()
events.set_event_loop(loop)

parser = argparse.ArgumentParser(description='LocalRunner for paperio')

for i in range(1, CONSTS.LR_CLIENTS_MAX_COUNT + 1):
    parser.add_argument('-p{}'.format(i), '--player{}'.format(i), type=str, nargs='?',
                        help='Path to executable with strategy for player {}'.format(i))
    parser.add_argument('--p{}l'.format(i), type=str, nargs='?', help='Path to log for player {}'.format(i))

parser.add_argument('-t', '--timeout', type=str, nargs='?', help='off/on timeout', default='on')
parser.add_argument('-s', '--scale', type=int, nargs='?', help='window scale (%%)', default=100)
parser.add_argument('--no-gui', dest='gui', action="store_false", help='Disable GUI')
parser.set_defaults(gui=True)

args = parser.parse_args()

connections_queue = asyncio.Queue()

async def connection_handler(client_reader, client_writer):
    client = TcpClient(client_reader, client_writer)
    connections_queue.put_nowait(client)

async def get_clients():
    clients = []
    for i in range(1, CONSTS.LR_CLIENTS_MAX_COUNT + 1):
        solution_id = f'player{i}'
        arg = getattr(args, solution_id)
        if arg:
            if arg == 'keyboard':
                client = KeyboardClient()
            elif arg == 'simple_bot':
                client = SimplePythonClient()
            elif arg == 'tcp':
                print(f'Waiting for player {i} to connect (port {TCP_PORT_NUMBER})...', file=sys.stderr)
                client = await connections_queue.get()
                print(f'Player {i} connected.', file=sys.stderr)
            else:
                client = FileClient(arg.split(), solution_id, getattr(args, 'p{}l'.format(i)))
            clients.append(client)
    return clients

loop.create_task(asyncio.start_server(connection_handler, '0.0.0.0', TCP_PORT_NUMBER))
clients = loop.run_until_complete(get_clients())

if not args.gui:
    game = Game(clients)
    game_future = asyncio.ensure_future(game.game_loop_wrapper())
    loop.run_until_complete(game_future)
    sys.exit(0)


import pyglet
from pyglet.gl import *
from pyglet.window import key

from game_objects.scene import Scene
from helpers import TERRITORY_CACHE, load_image

scene = Scene(args.scale)

for client in clients:
    if isinstance(client, KeyboardClient):
        client.set_window(scene.window)

if len(clients) == 0:
    clients.append(KeyboardClient(scene.window))

class Runner:
    @staticmethod
    def game_over_loop(dt):
        Runner.game.scene.clear()
        Runner.game.draw()

    @staticmethod
    def game_loop_wrapper(dt):
        is_game_over = loop.run_until_complete(Runner.game.game_loop())
        if is_game_over or (args.timeout == 'on' and Runner.game.tick >= CONSTS.MAX_TICK_COUNT):
            loop.run_until_complete(Runner.game.game_loop())
            Runner.game.send_game_end()
            Runner.game.game_save()
            Runner.stop_game()

    @staticmethod
    @scene.window.event
    def on_key_release(symbol, modifiers):
        if symbol == key.R:
            Runner.stop_game()
            TERRITORY_CACHE.clear()
            Runner.run_game()

    @staticmethod
    @scene.window.event
    def on_resize(width, height):
        (actual_width, actual_height) = scene.window.get_viewport_size()
        glViewport(0, 0, actual_width, actual_height)
        glMatrixMode(gl.GL_PROJECTION)
        glLoadIdentity()

        factScale = max(CONSTS.WINDOW_WIDTH / actual_width, CONSTS.WINDOW_HEIGHT / actual_height)
        xMargin = (actual_width * factScale - CONSTS.WINDOW_WIDTH) / 2
        yMargin = (actual_height * factScale - CONSTS.WINDOW_HEIGHT) / 2
        glOrtho(-xMargin, CONSTS.WINDOW_WIDTH + xMargin, -yMargin, CONSTS.WINDOW_HEIGHT + yMargin, -1, 1)
        glMatrixMode(gl.GL_MODELVIEW)
        return pyglet.event.EVENT_HANDLED

    @staticmethod
    def stop_game():
        pyglet.clock.schedule_interval(Runner.game_over_loop, 1 / 200)
        pyglet.clock.unschedule(Runner.game_loop_wrapper)

    @staticmethod
    def load_sprites():
        base_dir = os.path.dirname(os.path.realpath(__file__))
        absolute_path = os.path.join(base_dir, 'sprites')
        sprites = os.listdir(absolute_path)
        for sprite in sprites:
            if sprite.endswith('png'):
                load_image('sprites/{}'.format(sprite))

    @staticmethod
    def run_game():
        pyglet.clock.unschedule(Runner.game_over_loop)
        Runner.load_sprites()
        Runner.game = LocalGame(clients, scene, args.timeout == 'on')
        Runner.game.send_game_start()
        pyglet.clock.schedule_interval(Runner.game_loop_wrapper, 1 / 200)

Runner.run_game()
pyglet.app.run()
