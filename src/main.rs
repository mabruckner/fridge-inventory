extern crate iron;
extern crate router;

use iron::prelude::*;
use router::Router;

fn index(_: &mut Request) -> IronResult<Response>
{
    Ok(Response::with((iron::status::Ok,"Welcome to Fridge Inventory")))
}

fn hello_world(req: &mut Request) -> IronResult<Response>
{
    println!("{:?}",req);
    Ok(Response::with((iron::status::Ok,"Hello World")))
}

fn main()
{
    let mut router = Router::new();
    router.get("/",index);
    router.get("/hello",hello_world);

    Iron::new(router).http("localhost:4000").unwrap();
}
