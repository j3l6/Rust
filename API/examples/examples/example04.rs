use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde_json::json;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
enum Account {
    Free,
    Pro,
}

impl FromStr for Account {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "free" => Ok(Account::Free),
            "pro"  => Ok(Account::Pro),
            _      => Err(()),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct AccountParams {
    account: Account,
    months: i32,
}

// GET /account/{account}
async fn account(path: web::Path<AccountParams>) -> impl Responder {  // value enters the extractor, Serde turns it into into "free" or "Pro", web::Path<AccountType> returns web::Path wrapper 
    //let AccountType {account} = path.into_inner(); it accesses account field of AccountType and assings the name "account" to the obtained parameter, now you can use the "account" variable inside the code
    let params = path.into_inner(); // into_inner unwraps the path wrapper, now we have the internal instance and we can access its struct or fields
    
    if !(3..=12).contains(&params.months) { // "contains" check if month range is between 3-12, it returns false or true, !() inverts the result, "if" is only executed if "result" is true
        return HttpResponse::BadRequest()
            .body("`months` must be between 3 and 12");
    }
    
    HttpResponse::Ok().json(json!({
        "message": "Account created", 
        "account_type": params.account,
        "monts": params.months
    }))
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/account/{account}/{months}", web::get().to(account)) //call account function
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
