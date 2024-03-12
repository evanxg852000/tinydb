use std::{ops::Bound, str::FromStr, sync::Arc};


use crate::proto::{kvpb::{KvPair, RawDeleteRequest, RawDeleteResponse, RawGetRequest, RawGetResponse, RawPutRequest, RawPutResponse, RawScanRequest, RawScanResponse}, tinykv::tiny_kv_server::TinyKv};

use super::{storage::mutation::Mutation, ColumnFamily, Storage};

/// TinyKvService is a TinyKV server, it 'faces outwards', sending
/// and receiving messages from clients such as TinySQL.
pub struct TinyKvService {
    storage: Arc<dyn Storage>,
    // latches: Latches,
}

impl TinyKvService {
    pub fn new(storage: Arc<dyn Storage + Send + Sync>) -> Self {
        Self { storage }
    }
}

#[tonic::async_trait]
impl TinyKv for TinyKvService {

    async fn raw_get(&self, request:tonic::Request<RawGetRequest>) ->  Result<tonic::Response<RawGetResponse> ,tonic::Status> {
        let raw_req = request.into_inner();

        let cf = ColumnFamily::from_str(&raw_req.cf).unwrap();
        let value_opt = self.storage.get(cf, &raw_req.key).unwrap();
        
        let (value, not_found) = if let Some(value) = value_opt {
            (value, false)
        } else {
            (vec![], true)
        };

        Ok(tonic::Response::new(RawGetResponse{
            region_error: None, 
            error: "".to_string(),
            value,
            not_found,
        }))
    }

    async fn raw_put(&self, request:tonic::Request<RawPutRequest>) ->  Result<tonic::Response<RawPutResponse> ,tonic::Status> {
        let raw_req = request.into_inner();
        let mutation = Mutation::Put { 
            key: raw_req.key,
            value: raw_req.value,
            cf: ColumnFamily::from_str(&raw_req.cf).unwrap(),
        };
        self.storage.write(vec![mutation]).unwrap();

        Ok(tonic::Response::new(RawPutResponse{region_error: None, error: "".to_string()}))
    }

    async fn raw_delete(&self, request:tonic::Request<RawDeleteRequest>) ->  Result<tonic::Response<RawDeleteResponse> ,tonic::Status> {
        let raw_req = request.into_inner();
        let mutation = Mutation::Delete{ 
            key: raw_req.key,
            cf: ColumnFamily::from_str(&raw_req.cf).unwrap(),
        };
        self.storage.write(vec![mutation]).unwrap();

        Ok(tonic::Response::new(RawDeleteResponse{region_error: None, error: "".to_string()}))
    }

    async fn raw_scan(&self, request:tonic::Request<RawScanRequest>) ->  Result<tonic::Response<RawScanResponse> ,tonic::Status> {
        let raw_req = request.into_inner();
        let cf = ColumnFamily::from_str(&raw_req.cf).unwrap();
         
        let mut kvs = vec![];
        let scanner = self.storage.scan(cf, Bound::Included(raw_req.start_key), Bound::Unbounded).unwrap();
        for item in scanner.iter() {
            let (key, value) = item.unwrap();
            kvs.push(KvPair{
                error: None,
                key,
                value, 
            })
        }

        Ok(tonic::Response::new(RawScanResponse{
            region_error: None, 
            error: "".to_string(),
            kvs,
        }))
    }

}

