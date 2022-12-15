use crate::prelude::*;

pub type NodeId = String;
pub type Enr = enr::Enr<enr::CombinedKey>;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Bucket {
    #[serde(flatten)]
    node_ids: Vec<NodeId>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct KBucketsTable {
    #[serde(flatten)]
    buckets: Vec<Bucket>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeInfo {
    enr: Enr,
    node_id: NodeId,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoutingTableInfo {
    local_node_id: NodeId,
    buckets: KBucketsTable,
}

#[cfg(test)]
mod test {
    use crate::types::discv5::Enr;
    use std::net::Ipv4Addr;

    #[test]
    fn test_enr_ser_de() {
        let enr_base64 = r#""enr:-I24QAnHRBtPxxqnrZ0A9Xw1GV0cr3g178FcLutgd1DcG8a1FjOoRooOleI79K2NvTXYpOpkbe_NN-VqNZqS2a_Bo40BY4d0IDAuMS4wgmlkgnY0gmlwhH8AAAGJc2VjcDI1NmsxoQIJSs6oF8rPca9GjRV6tNaJ2YfZb5nNQjui2VUloBleH4N1ZHCCIyo""#;
        let expected_node_id = [
            176, 202, 35, 254, 68, 245, 224, 61, 174, 106, 81, 237, 41, 88, 144, 15, 55, 58, 125,
            119, 228, 39, 201, 211, 154, 95, 148, 198, 212, 185, 175, 219,
        ];
        let expected_ip4 = Some(Ipv4Addr::from([127, 0, 0, 1]));

        let enr: Enr = serde_json::from_str(enr_base64).unwrap();
        assert_eq!(enr.node_id(), expected_node_id);
        assert_eq!(enr.ip4(), expected_ip4);

        let decoded_enr = serde_json::to_string(&enr).unwrap();
        assert_eq!(decoded_enr, enr_base64);
    }
}
