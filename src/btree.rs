use std::cmp::Ordering;

use crate::internal::Oid;
use crate::internal::Xid;

trait Key : PartialOrd + Ord + PartialEq + Eq {
}

#[derive(Debug)]
struct ObjectMapKey {
    oid: Oid,
    xid: Xid,
}

impl Ord for ObjectMapKey {
    fn cmp(&self, other: &Self) -> Ordering {
        let order = self.oid.cmp(&other.oid);
        match order {
            Ordering::Equal => self.xid.cmp(&other.xid),
            _ => order,
        }
    }
}

impl PartialOrd for ObjectMapKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ObjectMapKey {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for ObjectMapKey {
}

impl Key for ObjectMapKey {
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_object_map_key_ordering() {
        let key1 = ObjectMapKey {
            oid: Oid(23),
            xid: Xid(17),
        };
        let key2 = ObjectMapKey {
            oid: Oid(23),
            xid: Xid(17),
        };
        let key_oid_less = ObjectMapKey {
            oid: Oid(21),
            xid: Xid(17),
        };
        let key_oid_greater = ObjectMapKey {
            oid: Oid(25),
            xid: Xid(17),
        };
        let key_xid_less = ObjectMapKey {
            oid: Oid(23),
            xid: Xid(16),
        };
        let key_xid_greater = ObjectMapKey {
            oid: Oid(23),
            xid: Xid(18),
        };
        let key_oid_less_xid_less = ObjectMapKey {
            oid: Oid(21),
            xid: Xid(16),
        };
        let key_oid_greater_xid_less = ObjectMapKey {
            oid: Oid(25),
            xid: Xid(16),
        };
        let key_oid_less_xid_greater = ObjectMapKey {
            oid: Oid(21),
            xid: Xid(18),
        };
        let key_oid_greater_xid_greater = ObjectMapKey {
            oid: Oid(25),
            xid: Xid(18),
        };
        assert_eq!(key1.cmp(&key2), Ordering::Equal);
        assert_eq!(key1.cmp(&key_oid_less), Ordering::Greater);
        assert_eq!(key1.cmp(&key_oid_greater), Ordering::Less);
        assert_eq!(key1.cmp(&key_xid_less), Ordering::Greater);
        assert_eq!(key1.cmp(&key_xid_greater), Ordering::Less);
        assert_eq!(key1.cmp(&key_oid_less_xid_less), Ordering::Greater);
        assert_eq!(key1.cmp(&key_oid_less_xid_greater), Ordering::Greater);
        assert_eq!(key1.cmp(&key_oid_greater_xid_less), Ordering::Less);
        assert_eq!(key1.cmp(&key_oid_greater_xid_greater), Ordering::Less);
    }

    #[test]
    fn test_object_map_key_equal() {
        let key1 = ObjectMapKey {
            oid: Oid(23),
            xid: Xid(17),
        };
        let key2 = ObjectMapKey {
            oid: Oid(23),
            xid: Xid(17),
        };
        let key3 = ObjectMapKey {
            oid: Oid(21),
            xid: Xid(17),
        };
        let key4 = ObjectMapKey {
            oid: Oid(23),
            xid: Xid(18),
        };
        assert_eq!(key1, key2);
        assert_ne!(key1, key3);
        assert_ne!(key1, key4);
        assert_ne!(key2, key3);
        assert_ne!(key2, key4);
        assert_ne!(key3, key4);
    }
}
