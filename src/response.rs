use crate::request::Request;
use crate::schema::responses;

#[derive(Debug, Serialize, Queryable, Insertable, Identifiable, Associations)]
#[belongs_to(Request)]
pub struct Response {
    id: String,
    data: String,
    time: String,
    request_id: String,
}
