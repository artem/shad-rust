use snapshot::{
    Dump, Error, NodeRecord, NodeSegmentRecord, ObjectId, ObjectType, PodRecord, PodSetRecord,
    ResourceTotals, Snapshot,
};

////////////////////////////////////////////////////////////////////////////////

trait DumpExt {
    fn pod_set_resource_requests(&self, pod_set_id: &ObjectId) -> ResourceTotals;
    fn node_resource_usage(&self, node_id: &ObjectId) -> ResourceTotals;
    fn node_segment_resource_totals(&self, node_segment_id: &ObjectId) -> ResourceTotals;
    fn node_segment_resource_usage(&self, node_segment_id: &ObjectId) -> ResourceTotals;
    fn node_segment_resource_requests(&self, node_segment_id: &ObjectId) -> ResourceTotals;
}

impl DumpExt for Dump {
    fn pod_set_resource_requests(&self, pod_set_id: &ObjectId) -> ResourceTotals {
        let mut totals = ResourceTotals::default();
        for pod in self.pods.iter() {
            if &pod.pod_set_id == pod_set_id {
                totals += pod.resource_requests;
            }
        }
        totals
    }

    fn node_resource_usage(&self, node_id: &ObjectId) -> ResourceTotals {
        let mut totals = ResourceTotals::default();
        for pod in self.pods.iter() {
            if pod.node_id.as_ref() == Some(node_id) {
                totals += pod.resource_requests;
            }
        }
        totals
    }

    fn node_segment_resource_totals(&self, node_segment_id: &ObjectId) -> ResourceTotals {
        let mut totals = ResourceTotals::default();
        for node in self.nodes.iter() {
            if &node.node_segment_id == node_segment_id {
                totals += node.resource_totals;
            }
        }
        totals
    }

    fn node_segment_resource_usage(&self, node_segment_id: &ObjectId) -> ResourceTotals {
        let mut totals = ResourceTotals::default();
        for node in self.nodes.iter() {
            if &node.node_segment_id == node_segment_id {
                totals += self.node_resource_usage(&node.id);
            }
        }
        totals
    }

    fn node_segment_resource_requests(&self, node_segment_id: &ObjectId) -> ResourceTotals {
        let mut totals = ResourceTotals::default();
        for pod_set in self.pod_sets.iter() {
            if &pod_set.node_segment_id == node_segment_id {
                totals += self.pod_set_resource_requests(&pod_set.id);
            }
        }
        totals
    }
}

////////////////////////////////////////////////////////////////////////////////

fn check_error<T>(res: Result<T, Error>, expected_err: Error) {
    match res {
        Ok(_) => panic!("expected {:?}, got Ok", expected_err),
        Err(err) => assert_eq!(err, expected_err),
    }
}

////////////////////////////////////////////////////////////////////////////////

#[test]
fn test_simple() {
    let dump = Dump {
        node_segments: vec![NodeSegmentRecord { id: 1 }, NodeSegmentRecord { id: 2 }],
        nodes: vec![
            NodeRecord {
                id: 1,
                node_segment_id: 1,
                resource_totals: ResourceTotals {
                    cpu: 16,
                    memory: 32,
                    disk_capacity: 1000,
                },
            },
            NodeRecord {
                id: 2,
                node_segment_id: 1,
                resource_totals: ResourceTotals {
                    cpu: 24,
                    memory: 20,
                    disk_capacity: 2000,
                },
            },
            NodeRecord {
                id: 101,
                node_segment_id: 2,
                resource_totals: ResourceTotals {
                    cpu: 40,
                    memory: 30,
                    disk_capacity: 1500,
                },
            },
        ],
        pod_sets: vec![
            PodSetRecord {
                id: 1,
                node_segment_id: 1,
            },
            PodSetRecord {
                id: 1000,
                node_segment_id: 2,
            },
        ],
        pods: vec![
            PodRecord {
                id: 301,
                pod_set_id: 1,
                node_id: Some(1),
                resource_requests: ResourceTotals {
                    cpu: 4,
                    memory: 16,
                    disk_capacity: 100,
                },
            },
            PodRecord {
                id: 302,
                pod_set_id: 1,
                node_id: Some(1),
                resource_requests: ResourceTotals {
                    cpu: 6,
                    memory: 10,
                    disk_capacity: 200,
                },
            },
            PodRecord {
                id: 303,
                pod_set_id: 1,
                node_id: None,
                resource_requests: ResourceTotals {
                    cpu: 30,
                    memory: 60,
                    disk_capacity: 250,
                },
            },
            PodRecord {
                id: 401,
                pod_set_id: 1000,
                node_id: Some(101),
                resource_requests: ResourceTotals {
                    cpu: 10,
                    memory: 20,
                    disk_capacity: 450,
                },
            },
        ],
    };

    let snap = Snapshot::new(&dump).unwrap();

    assert_eq!(snap.pods().len(), dump.pods.len());
    assert_eq!(snap.pod_sets().len(), dump.pod_sets.len());
    assert_eq!(snap.nodes().len(), dump.nodes.len());
    assert_eq!(snap.node_segments().len(), dump.node_segments.len());

    for pod_rec in dump.pods.iter() {
        let pod = snap.get_pod(&pod_rec.id).unwrap();
        assert_eq!(pod.id, pod_rec.id);
        assert_eq!(pod.resource_requests, pod_rec.resource_requests);
        assert_eq!(pod.pod_set().id, pod_rec.pod_set_id);
    }

    for pod_set_rec in dump.pod_sets.iter() {
        let pod_set = snap.get_pod_set(&pod_set_rec.id).unwrap();
        assert_eq!(pod_set.id, pod_set_rec.id);
        assert_eq!(pod_set.node_segment().id, pod_set_rec.node_segment_id);

        let mut resource_requests = ResourceTotals::default();
        for pod in pod_set.pods() {
            resource_requests += pod.resource_requests;
        }

        assert_eq!(
            resource_requests,
            dump.pod_set_resource_requests(&pod_set_rec.id)
        );
        assert_eq!(pod_set.resource_requests, resource_requests);
    }

    for node_rec in dump.nodes.iter() {
        let node = snap.get_node(&node_rec.id).unwrap();
        assert_eq!(node.id, node_rec.id);
        assert_eq!(node.node_segment().id, node_rec.node_segment_id);
        assert_eq!(node.resource_totals, node_rec.resource_totals);

        let mut resource_usage = ResourceTotals::default();
        for pod in node.pods() {
            resource_usage += pod.resource_requests;
        }

        assert_eq!(resource_usage, dump.node_resource_usage(&node_rec.id));
        assert_eq!(node.resource_usage, resource_usage);
    }

    for node_segment_rec in dump.node_segments.iter() {
        let node_segment = snap.get_node_segment(&node_segment_rec.id).unwrap();
        assert_eq!(node_segment.id, node_segment_rec.id);

        let mut resource_totals = ResourceTotals::default();
        let mut resource_usage = ResourceTotals::default();
        for node in node_segment.nodes() {
            resource_totals += node.resource_totals;
            resource_usage += node.resource_usage;
        }

        assert_eq!(
            resource_totals,
            dump.node_segment_resource_totals(&node_segment_rec.id)
        );
        assert_eq!(
            resource_usage,
            dump.node_segment_resource_usage(&node_segment_rec.id)
        );

        let mut resource_requests = ResourceTotals::default();
        for pod_set in node_segment.pod_sets() {
            resource_requests += pod_set.resource_requests;
        }

        assert_eq!(
            resource_requests,
            dump.node_segment_resource_requests(&node_segment_rec.id)
        );
    }
}

#[test]
fn test_missing_node_segment() {
    let dump = Dump {
        node_segments: vec![NodeSegmentRecord { id: 123 }],
        nodes: vec![NodeRecord {
            id: 24,
            node_segment_id: 124,
            resource_totals: Default::default(),
        }],
        ..Default::default()
    };

    let res = Snapshot::new(&dump);
    check_error(
        res,
        Error::MissingObject {
            id: 124,
            ty: ObjectType::NodeSegment,
        },
    );
}

#[test]
fn test_missing_node() {
    let dump = Dump {
        node_segments: vec![NodeSegmentRecord { id: 123 }],
        nodes: vec![NodeRecord {
            id: 24,
            node_segment_id: 123,
            resource_totals: Default::default(),
        }],
        pod_sets: vec![PodSetRecord {
            id: 10,
            node_segment_id: 123,
        }],
        pods: vec![PodRecord {
            id: 134,
            pod_set_id: 10,
            node_id: Some(34),
            resource_requests: Default::default(),
        }],
    };

    let res = Snapshot::new(&dump);
    check_error(
        res,
        Error::MissingObject {
            id: 34,
            ty: ObjectType::Node,
        },
    );
}

#[test]
fn test_missing_pod_set() {
    let dump = Dump {
        pods: vec![PodRecord {
            id: 134,
            pod_set_id: 10,
            node_id: None,
            resource_requests: Default::default(),
        }],
        ..Default::default()
    };

    let res = Snapshot::new(&dump);
    check_error(
        res,
        Error::MissingObject {
            id: 10,
            ty: ObjectType::PodSet,
        },
    );
}

#[test]
fn test_duplicate_node_segment() {
    let dump = Dump {
        node_segments: vec![NodeSegmentRecord { id: 2 }, NodeSegmentRecord { id: 2 }],
        ..Default::default()
    };

    let res = Snapshot::new(&dump);
    check_error(
        res,
        Error::DuplicateObject {
            id: 2,
            ty: ObjectType::NodeSegment,
        },
    );
}

#[test]
fn test_duplicate_node() {
    let dump = Dump {
        node_segments: vec![NodeSegmentRecord { id: 1 }, NodeSegmentRecord { id: 2 }],
        nodes: vec![
            NodeRecord {
                id: 10,
                node_segment_id: 1,
                resource_totals: Default::default(),
            },
            NodeRecord {
                id: 10,
                node_segment_id: 2,
                resource_totals: Default::default(),
            },
        ],
        ..Default::default()
    };

    let res = Snapshot::new(&dump);
    check_error(
        res,
        Error::DuplicateObject {
            id: 10,
            ty: ObjectType::Node,
        },
    );
}

#[test]
fn test_duplicate_pod_set() {
    let dump = Dump {
        node_segments: vec![NodeSegmentRecord { id: 1 }, NodeSegmentRecord { id: 2 }],
        pod_sets: vec![
            PodSetRecord {
                id: 15,
                node_segment_id: 1,
            },
            PodSetRecord {
                id: 15,
                node_segment_id: 2,
            },
        ],
        ..Default::default()
    };

    let res = Snapshot::new(&dump);
    check_error(
        res,
        Error::DuplicateObject {
            id: 15,
            ty: ObjectType::PodSet,
        },
    );
}

#[test]
fn test_duplicate_pod() {
    let dump = Dump {
        node_segments: vec![NodeSegmentRecord { id: 1 }],
        pod_sets: vec![
            PodSetRecord {
                id: 25,
                node_segment_id: 1,
            },
            PodSetRecord {
                id: 45,
                node_segment_id: 1,
            },
        ],
        pods: vec![
            PodRecord {
                id: 100,
                pod_set_id: 25,
                node_id: None,
                resource_requests: Default::default(),
            },
            PodRecord {
                id: 100,
                pod_set_id: 45,
                node_id: None,
                resource_requests: Default::default(),
            },
        ],
        ..Default::default()
    };

    let res = Snapshot::new(&dump);
    check_error(
        res,
        Error::DuplicateObject {
            id: 100,
            ty: ObjectType::Pod,
        },
    );
}

#[test]
fn test_no_reborrowing() {
    let dump = Dump {
        node_segments: vec![NodeSegmentRecord { id: 1 }],
        nodes: vec![NodeRecord {
            id: 1,
            node_segment_id: 1,
            resource_totals: Default::default(),
        }],
        ..Default::default()
    };

    let snap = Snapshot::new(&dump).unwrap();
    let node = snap.get_node(&dump.nodes[0].id).unwrap();
    let node_segment = node.node_segment();
    drop(node);
    assert_eq!(node_segment.id, dump.node_segments[0].id);
}

#[cfg(feature = "test_lifetime")]
#[test]
fn test_lifetime() {
    let dump = Dump {
        node_segments: vec![NodeSegmentRecord { id: 1 }],
        nodes: vec![NodeRecord {
            id: 1,
            node_segment_id: 1,
            resource_totals: Default::default(),
        }],
        ..Default::default()
    };

    let snap = Snapshot::new(&dump).unwrap();
    let node = snap.get_node(&dump.nodes[0].id).unwrap();
    let node_segment = node.node_segment();

    drop(snap);
    drop(dump);

    assert_eq!(node_segment.id, node_segment.id);
}
