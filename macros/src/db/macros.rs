
#[macro_export]
macro_rules! filter {
    ($query:expr, $(($column:expr, @$expression_method:ident, $param:expr)),*) => {
        {
            $(
                if let Some(item) = $param {
                    let filter = filter!($column, @$expression_method, item);
                    $query = $query.filter(filter);
                }
            )*

            $query
        }
    };
    ($column:expr, @like, $item:expr) => { $column.like($item) };
    ($column:expr, @ge, $item:expr) => { $column.ge($item) };
    ($column:expr, @le, $item:expr) => { $column.le($item) };
}

#[macro_export]
macro_rules! sort_by {
    ($query:expr, $sort_by:expr, $(($param:expr, $column:expr)),*) => {
        {
            if let Some(sort_by) = $sort_by {
                $query = match sort_by.as_ref() {
                    $(
                        $param => $query.order($column.asc()),
                        concat!($param, ".asc") => $query.order($column.asc()),
                        concat!($param, ".desc") => $query.order($column.desc()),
                    )*
                    _ => $query,
                }
            }

            $query
        }
    };
}
