pub struct PersonAnalyzer;

use crate::core::lists::names::a::NAMES_A;
use crate::core::lists::names::b::NAMES_B;
use crate::core::lists::names::c::NAMES_C;
use crate::core::lists::names::d::NAMES_D;
use crate::core::lists::names::e::NAMES_E;
use crate::core::lists::names::f::NAMES_F;
use crate::core::lists::names::g::NAMES_G;
use crate::core::lists::names::h::NAMES_H;
use crate::core::lists::names::i::NAMES_I;
use crate::core::lists::names::j::NAMES_J;
use crate::core::lists::names::k::NAMES_K;
use crate::core::lists::names::l::NAMES_L;
use crate::core::lists::names::m::NAMES_M;
use crate::core::lists::names::n::NAMES_N;
use crate::core::lists::names::o::NAMES_O;
use crate::core::lists::names::p::NAMES_P;
use crate::core::lists::names::q::NAMES_Q;
use crate::core::lists::names::r::NAMES_R;
use crate::core::lists::names::s::NAMES_S;
use crate::core::lists::names::t::NAMES_T;
use crate::core::lists::names::u::NAMES_U;
use crate::core::lists::names::v::NAMES_V;
use crate::core::lists::names::w::NAMES_W;
use crate::core::lists::names::x::NAMES_X;
use crate::core::lists::names::y::NAMES_Y;
use crate::core::lists::names::z::NAMES_Z;

impl PersonAnalyzer {

    pub fn analysis(&self, text_to_analyze: &str) -> String {
        let pattern = "PERSON".to_string();
        let names: Vec<String> = text_to_analyze.split_whitespace().map(String::from).collect();
        let mut patter_confirm = "";
                
        for mut name in names {
            name = name.to_lowercase();

            match name.chars().next() {
                Some('a') => {
                    if NAMES_A.contains(&name.as_str()) {
                    println!("Name: {}", name);
                    patter_confirm = "ok";
                    break;
                }}
                Some('b') => {
                    if NAMES_B.contains(&name.as_str()) {
                    println!("Name: {}", name);
                    patter_confirm = "ok";
                    break;
                }}
                Some('c') => {
                    if NAMES_C.contains(&name.as_str()) {
                    println!("Name: {}", name);
                    patter_confirm = "ok";
                    break;
                }}
                Some('d') => {
                    if NAMES_D.contains(&name.as_str()) {
                    println!("Name: {}", name);
                    patter_confirm = "ok";
                    break;
                }}
                Some('e') => {
                    if NAMES_E.contains(&name.as_str()) {
                    println!("Name: {}", name);
                    patter_confirm = "ok";
                    break;
                }}
                Some('f') => {
                    if NAMES_F.contains(&name.as_str()) {
                    println!("Name: {}", name);
                    patter_confirm = "ok";
                    break;
                }}
                Some('g') => {
                    if NAMES_G.contains(&name.as_str()) {
                    println!("Name: {}", name);
                    patter_confirm = "ok";
                    break;
                }}
                Some('h') => {
                    if NAMES_H.contains(&name.as_str()) {
                    println!("Name: {}", name);
                    patter_confirm = "ok";
                    break;
                }}
                Some('i') => {
                    if NAMES_I.contains(&name.as_str()) {
                    println!("Name: {}", name);
                    patter_confirm = "ok";
                    break;
                }}
                Some('j') => {
                    if NAMES_J.contains(&name.as_str()) {
                    println!("Name: {}", name);
                    patter_confirm = "ok";
                    break;
                }}
                Some('k') => {
                    if NAMES_K.contains(&name.as_str()) {
                    println!("Name: {}", name);
                    patter_confirm = "ok";
                    break;
                }}
                Some('l') => {
                    if NAMES_L.contains(&name.as_str()) {
                    println!("Name: {}", name);
                    patter_confirm = "ok";
                    break;
                }}
                Some('m') => {
                    if NAMES_M.contains(&name.as_str()) {
                    println!("Name: {}", name);
                    patter_confirm = "ok";
                    break;
                }}
                Some('n') => {
                    if NAMES_N.contains(&name.as_str()) {
                    println!("Name: {}", name);
                    patter_confirm = "ok";
                    break;
                }}
                Some('o') => {
                    if NAMES_O.contains(&name.as_str()) {
                    println!("Name: {}", name);
                    patter_confirm = "ok";
                    break;
                }}
                Some('p') => {
                    if NAMES_P.contains(&name.as_str()) {
                    println!("Name: {}", name);
                    patter_confirm = "ok";
                    break;
                }}
                Some('q') => {
                    if NAMES_Q.contains(&name.as_str()) {
                    println!("Name: {}", name);
                    patter_confirm = "ok";
                    break;
                }}
                Some('r') => {
                    if NAMES_R.contains(&name.as_str()) {
                    println!("Name: {}", name);
                    patter_confirm = "ok";
                    break;
                }}
                Some('s') => {
                    if NAMES_S.contains(&name.as_str()) {
                    println!("Name: {}", name);
                    patter_confirm = "ok";
                    break;
                }}
                Some('t') => {
                    if NAMES_T.contains(&name.as_str()) {
                    println!("Name: {}", name);
                    patter_confirm = "ok";
                    break;
                }}
                Some('u') => {
                    if NAMES_U.contains(&name.as_str()) {
                    println!("Name: {}", name);
                    patter_confirm = "ok";
                    break;
                }}
                Some('v') => {
                    if NAMES_V.contains(&name.as_str()) {
                    println!("Name: {}", name);
                    patter_confirm = "ok";
                    break;
                }}
                Some('w') => {
                    if NAMES_W.contains(&name.as_str()) {
                    println!("Name: {}", name);
                    patter_confirm = "ok";
                    break;
                }}
                Some('x') => {
                    if NAMES_X.contains(&name.as_str()) {
                    println!("Name: {}", name);
                    patter_confirm = "ok";
                    break;
                }}
                Some('y') => {
                    if NAMES_Y.contains(&name.as_str()) {
                    println!("Name: {}", name);
                    patter_confirm = "ok";
                    break;
                }}
                Some('z') => {
                    if NAMES_Z.contains(&name.as_str()) {
                    println!("Name: {}", name);
                    patter_confirm = "ok";
                    break; 
                }}

                _ => {}
            }
        }
        
        if patter_confirm.is_empty() {
            String::new()
        } else {
            pattern
        }
    }
}