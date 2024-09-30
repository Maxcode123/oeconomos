diesel::table! {
    transaction_categories (id) {
        id -> Integer,
        name -> VarChar
    }
}

diesel::table! {
    transactions (id) {
        id -> Integer,
        date -> Date,
        transaction_number -> VarChar,
        amount -> Float,
        category_id -> Integer
    }
}

diesel::joinable!(transactions -> transaction_categories (category_id));
