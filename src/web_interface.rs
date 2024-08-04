use rocket::{ self, get, post, routes };
use rocket::response::content::RawHtml;
use rocket::form::Form;
use crate::search::SearchEngine;
