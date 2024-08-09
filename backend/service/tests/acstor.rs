mod prepare_acstor;

use prepare_acstor::prepare_mock_db;
use service::Query;

#[tokio::test]
async fn main() {
    let db = &prepare_mock_db().await;

    {
        let note = Query::find_user_by_id(db, 1).await.unwrap().unwrap();

        assert_eq!(note.id, 2);
    }
}
