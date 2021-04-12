// DataTable createDataTable() => DataTable([
//       ["Browser", "Share"],
//       ["Chrome", 35],
//       ["IE", 30],
//       ["Firefox", 20]
//     ]);

//   test("columns", || {
//     let table = createDataTable();
//     expect(table.columns.len(), equals(2));
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
//     expect(table.rows.len(), equals(3));
//     expect(table.rows[1].toList(), orderedEquals(["IE", 30]));
//   });

//   test("columns.insert", || {
//     let table = createDataTable();
//     table.columns.insert(1, DataColumn("Latest Version", num));
//     expect(table.columns.len(), equals(3));
//     expect(table.columns[1].name, equals("Latest Version"));
//   });

//   test("rows.add", || {
//     let table = createDataTable();
//     table.rows.add(["Opera", 10, "discarded"]);
//     expect(table.rows.len(), equals(4));
//     expect(table.rows.last.toList(), orderedEquals(["Opera", 10]));
//   });

//   test("rows.remove_range", || {
//     let table = createDataTable();
//     table.rows.remove_range(0, 3);
//     expect(table.rows, isEmpty);
//   });

//   test("cells", || {
//     let table = createDataTable();
//     expect(table.rows[0][0], equals("Chrome"));
//     table.rows[0][0] = "Unknown";
//     expect(table.rows[0][0], equals("Unknown"));
//   });
