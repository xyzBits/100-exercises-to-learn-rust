pub fn example() {
    // Trying to get the size of a str (or any other DST)
    // via `std::mem::size_of` will result in a compile-time error.
    //
    // TODO: Comment out the following line and move on to the next exercise.
    // doesn't have a size known at compile-time
    // std::mem::size_of::<str>();
    std::mem::size_of::<&str>();
}
