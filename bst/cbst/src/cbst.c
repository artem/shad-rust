#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct node {
    int64_t key;
    struct node* left;
    struct node* right;
};

static void node_free(struct node* node) {
    if (node) {
        node_free(node->left);
        node_free(node->right);
        free(node);
    }
}

struct cbst {
    struct node* head;
    size_t len;
};

void cbst_init(struct cbst* cbst) {
    cbst->head = NULL;
    cbst->len = 0;
}

void cbst_free(struct cbst* cbst) {
    node_free(cbst->head);
}

static struct node** cbst_find_node(struct cbst* cbst, int64_t key) {
    struct node** node = &cbst->head;
    while (*node) {
        int64_t node_key = (*node)->key;
        if (node_key == key) {
            return node;
        }
        if (node_key < key) {
            node = &(*node)->right;
        } else {
            node = &(*node)->left;
        }
    }
    return node;
}

bool cbst_contains(struct cbst* cbst, int64_t key) {
    struct node** node = cbst_find_node(cbst, key);
    return *node;
}

bool cbst_insert(struct cbst* cbst, int64_t key) {
    struct node** node = cbst_find_node(cbst, key);
    if (*node) {
        return false;
    }

    struct node* new = malloc(sizeof(struct node));
    new->key = key;
    new->left = NULL;
    new->right = NULL;
    *node = new;

    ++cbst->len;
    return true;
}

static struct node** cbst_rotate_left(struct node** root) {
    struct node* node = *root;
    struct node* right = node->right;
    node->right = right->left;
    right->left = node;
    *root = right;
    return &right->left;
}

bool cbst_remove(struct cbst* cbst, int64_t key) {
    struct node** node = cbst_find_node(cbst, key);
    if (!*node) {
        return false;
    }

    while ((*node)->right) {
        node = cbst_rotate_left(node);
    }

    struct node* junk = *node;
    *node = junk->left;
    free(junk);

    --cbst->len;
    return true;
}