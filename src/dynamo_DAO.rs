use chrono::Local;
use aws_sdk_dynamodb::{model::AttributeValue, Client, Error};

pub async fn add_item(client: &Client, item: Item, table: &String) -> Result<(), Error> {
    let ulid = AttributeValue::S(item.ulid);
    let ms_name = AttributeValue::S(item.ms_name);

    let request = client
        .put_item()
        .table_name(table)
        .item("ulid", ulid)
        .item("ms_name", ms_name);
    println!("Executing request [{request:?}] to add item...");

    let resp = request.send().await?;

    let attributes = resp.attributes().unwrap();

    println!(
        "Added user {:?}, {:?} {:?}, age {:?} as {:?} user",
        attributes.get("ulid"),
        attributes.get("ms_name")
    );

    Ok(())
}

pub async fn delete_item(client: &Client, table: &str, key: &str, value: &str, ) -> Result<(), Error> {
    match client
        .delete_item()
        .table_name(table)
        .key(key, AttributeValue::S(value.into()))
        .send()
        .await
    {
        Ok(_) => {
            println!("Deleted item from table");
            Ok(())
        }
        Err(e) => Err(Error::Unhandled(Box::new(e))),
    }
}

// example querying the table
pub async fn delete_expired_ulids(client: &Client, table_name: &str, year: u16, ) -> Result<Vec<>, Error> {

    let current_time = Local::now();
    filter = "#expirationDate < :currentTime";
    let results = client
        .query()
        .table_name(table_name)
        .key_condition_expression("#msName = ms_name")
        .expression_attribute_names("#expirationDate:expirationDate", "#msName:ms_name")
        .expression_attribute_values(":yyyy", AttributeValue::N(year.to_string()))
        .expression_attribute_values(":msName", AttributeValueMemberS{Value: msName}) // all expired ULIDs from the msName
        .expression_attribute_values(":currentTime", AttributeValueMemberS{Value: current_time})
        .filter_expression(filter)// all expired ULIDs from the msName
        .send()
        .await?;

    // Some represent the possibility of something failing if does then it returns NONE else
    // it returns the value needed in this case results.items
    if let Some(items) = results.items {
        let expired = items.iter().map(|v| v.into()).collect();
        Ok(expired)
    } else {
        // Ok represents success in containing a value.
        Ok(vec![]) // empty vector 
    }

    for item in results.items {
        delete_item(client,table_name, item, value)
    }


}



