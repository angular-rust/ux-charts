#![allow(unused_imports)]
#![allow(unused_variables)]

// use std::any::Any;

use ux_charts::*;
use ux_primitives::datatable::*;

// let random = Random();

// i64 rand(i64 min, i64 max) => random.nextInt(max - min) + min;

// fn createContainer() -> Element {
// //   let e = DivElement()
// //     ..style.height = "400px"
// // //    ..style.width = "800px"
// //     ..style.maxWidth = "100%"
// //     ..style.marginBottom = "50px";
// //   document.body.append(e);
// //   return e;
// }

// February
fn main() {
    let metadata = vec![
        Channel {
            name: "Categories",
            tag: 0,
            visible: true,
        },
        Channel {
            name: "Long series name",
            tag: 1,
            visible: true,
        },
        Channel {
            name: "Series 2",
            tag: 2,
            visible: true,
        },
        Channel {
            name: "Series 3",
            tag: 3,
            visible: true,
        },
    ];

    // Zero stream tag is allways metric
    let mut frames = vec![DataFrame {
        metric: "January",
        data: [(1, 1), (2, 3), (3, 5)].iter().cloned().collect(),
    }];

    frames.push(DataFrame {
        metric: "February",
        data: [(1, 3), (2, 4), (3, 6)].iter().cloned().collect(),
    });

    frames.push(DataFrame {
        metric: "March",
        data: [(1, 4), (2, 3), (3, 1)].iter().cloned().collect(),
    });

    // let skip one stream flow
    frames.push(DataFrame {
        metric: "April",
        data: [(2, 5), (3, 1)].iter().cloned().collect(),
    });

    frames.push(DataFrame {
        metric: "May",
        data: [(1, 3), (2, 4), (3, 2)].iter().cloned().collect(),
    });

    frames.push(DataFrame {
        metric: "June",
        data: [(1, 5), (2, 10), (3, 4)].iter().cloned().collect(),
    });

    frames.push(DataFrame {
        metric: "July",
        data: [(1, 4), (2, 12), (3, 8)].iter().cloned().collect(),
    });

    frames.push(DataFrame {
        metric: "August",
        data: [(1, 1), (2, 3), (3, 5)].iter().cloned().collect(),
    });

    frames.push(DataFrame {
        metric: "September",
        data: [(1, 3), (2, 4), (3, 6)].iter().cloned().collect(),
    });

    frames.push(DataFrame {
        metric: "October",
        data: [(1, 4), (2, 3), (3, 1)].iter().cloned().collect(),
    });

    frames.push(DataFrame {
        metric: "November",
        data: [(2, 5), (3, 1)].iter().cloned().collect(),
    });

    frames.push(DataFrame {
        metric: "December",
        data: [(1, 3), (2, 4), (3, 2)].iter().cloned().collect(),
    });

    let stream = DataStream::new(metadata, frames);

    // let changeDataButton = ButtonElement()..text = "Change data";
    // document.body.append(changeDataButton);

    // let insertRemoveColumnButton = ButtonElement()
    //   ..text = "Insert/remove data column";
    // document.body.append(insertRemoveColumnButton);

    // let insertRemoveRowButton = ButtonElement()..text = "Insert/remove data row";
    // document.body.append(insertRemoveRowButton);

    // let container = createContainer();

    // let options = {
    //   "animation": {
    //     "onEnd": () {
    //       changeDataButton.disabled = false;
    //       insertRemoveColumnButton.disabled = false;
    //       insertRemoveRowButton.disabled = false;
    //     }
    //   },
    //   "series": {
    //     "labels": {"enabled": true}
    //   },
    //   "xAxis": {
    //     "crosshair": {"enabled": true},
    //     "labels": {"maxRotation": 90, "minRotation": 0}
    //   },
    //   "yAxis": {"minValue": 0, "minInterval": 5},
    //   "title": {"text": "Bar Chart Demo"},
    //   "tooltip": {"valueFormatter": (value) => "$value units"}
    // };

    // let chart = BarChart::new(Default::default());
    // chart.set_stream(stream);
    // chart.draw(table, options);

    // fn disableAllButtons() {
    //   changeDataButton.disabled = true;
    //   insertRemoveColumnButton.disabled = true;
    //   insertRemoveRowButton.disabled = true;
    // }

    // changeDataButton.onClick.listen((_) {
    //   disableAllButtons();
    //   for (let row in table.rows) {
    //     for (let i = 1; i < table.columns.length; i++) {
    //       row[i] = rand(2, 20);
    //     }
    //   }
    //   chart.update();
    // });

    // let insertColumn = true;
    // insertRemoveColumnButton.onClick.listen((_) {
    //   disableAllButtons();
    //   if (insertColumn) {
    //     table.columns.insert(2, DataColumn("New series", num));
    //     for (let row in table.rows) {
    //       row[2] = rand(2, 20);
    //     }
    //   } else {
    //     table.columns.removeAt(2);
    //   }
    //   insertColumn = !insertColumn;
    //   chart.update();
    // });

    // let insertRow = true;
    // insertRemoveRowButton.onClick.listen((_) {
    //   disableAllButtons();
    //   if (insertRow) {
    //     let values = <dynamic>["New"];
    //     for (let i = 1; i < table.columns.length; i++) {
    //       values.add(rand(2, 20));
    //     }
    //     table.rows.insert(2, values);
    //   } else {
    //     table.rows.removeAt(2);
    //   }
    //   insertRow = !insertRow;
    //   chart.update();
    // });
    unimplemented!()
}
