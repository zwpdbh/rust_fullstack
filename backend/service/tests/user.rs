mod prepare_user;

use prepare_user::prepare_mock_db;
use service::Query;

#[tokio::test]
async fn main() {
    let db = &prepare_mock_db();

    {
        let note = Query::find_user_by_id(db, 1).await.unwrap().unwrap();

        assert_eq!(note.id, 1);
    }
}
