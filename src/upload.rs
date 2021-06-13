use rusoto_core::Region;
use rusoto_s3::{PutObjectRequest, S3Client, S3};
use std::fs;

pub fn upload(path: &str) {
    let s3_client = S3Client::new(Region::ApSoutheast1);

    let file = fs::OpenOptions::new().open(path).unwrap();

    let tokio_file = tokio::fs::File::from_std(file);
    
    // let byte_stream =
    //     codec::FramedRead::new(tokio_file, codec::BytesCodec::new()).map(|r| r.as_ref().to_vec());

    // let result = s3_client.put_object(PutObjectRequest {
    //     bucket: String::from("dbdatadumps"),
    //     key: String::from("todos"),
    //     body:
    // });
}
