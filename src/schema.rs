table! {
    masses (id) {
        id -> Nullable<Integer>,
        name -> Varchar,
        pos_x -> Double,
        pos_y -> Double,
        pos_z -> Double,
        vel_x -> Double,
        vel_y -> Double,
        vel_z -> Double,
        type_data -> Jsonb,
    }
}
