mod animation;
pub use animation::*;

mod backend;
pub use backend::*;

mod canvas;
pub use canvas::*;

mod color;
pub use color::*;

mod text;
pub use text::*;

mod math;
pub use math::*;

mod bar;
pub use bar::*;

mod base;
pub use base::*;

mod datatable;
pub use datatable::*;

mod gauge;
pub use gauge::*;

mod line;
pub use gauge::*;

mod pie;
pub use pie::*;

mod radar;
pub use radar::*;

mod utils;
pub use utils::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

// DataTable createDataTable() => DataTable([
//       ["Browser", "Share"],
//       ["Chrome", 35],
//       ["IE", 30],
//       ["Firefox", 20]
//     ]);


//   test("columns", || {
//     let table = createDataTable();
//     expect(table.columns.length, equals(2));
//     expect(table.columns[0].name, equals("Browser"));
//   });

//   test("getColumnIndexByName", || {
//     let table = createDataTable();
//     expect(table.getColumnIndexByName("Share"), equals(1));
//     expect(table.getColumnIndexByName("X"), equals(-1));
//   });

//   test("getColumnValues", || {
//     let table = createDataTable();
//     expect(table.getColumnValues(1), orderedEquals([35, 30, 20]));
//   });

//   test("rows", || {
//     let table = createDataTable();
//     expect(table.rows.length, equals(3));
//     expect(table.rows[1].toList(), orderedEquals(["IE", 30]));
//   });

//   test("columns.insert", || {
//     let table = createDataTable();
//     table.columns.insert(1, DataColumn("Latest Version", num));
//     expect(table.columns.length, equals(3));
//     expect(table.columns[1].name, equals("Latest Version"));
//   });

//   test("rows.add", || {
//     let table = createDataTable();
//     table.rows.add(["Opera", 10, "discarded"]);
//     expect(table.rows.length, equals(4));
//     expect(table.rows.last.toList(), orderedEquals(["Opera", 10]));
//   });

//   test("rows.removeRange", || {
//     let table = createDataTable();
//     table.rows.removeRange(0, 3);
//     expect(table.rows, isEmpty);
//   });

//   test("cells", || {
//     let table = createDataTable();
//     expect(table.rows[0][0], equals("Chrome"));
//     table.rows[0][0] = "Unknown";
//     expect(table.rows[0][0], equals("Unknown"));
//   });