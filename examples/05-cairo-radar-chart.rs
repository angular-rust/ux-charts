#![allow(unused_imports)]
#![allow(unused_variables)]

use ux_charts::*;
use ux_primitives::datatable::*;

fn main() {
    let metadata = vec![
        Channel {
            name: "Categories",
            tag: 0,
            visible: true,
        },
        Channel {
            name: "Series 1",
            tag: 1,
            visible: true,
        },
    ];

    // Zero stream tag is allways metric
    let mut frames = vec![DataFrame {
        metric: "Monday",
        data: [(1, 8)].iter().cloned().collect(),
    }];

    frames.push(DataFrame {
        metric: "Tuesday",
        data: [(1, 17)].iter().cloned().collect(),
    });

    frames.push(DataFrame {
        metric: "Wednesday",
        data: [(1, 7)].iter().cloned().collect(),
    });

    frames.push(DataFrame {
        metric: "Thursday",
        data: [(1, 16)].iter().cloned().collect(),
    });

    frames.push(DataFrame {
        metric: "Friday",
        data: [(1, 12)].iter().cloned().collect(),
    });

    frames.push(DataFrame {
        metric: "Saturday",
        data: [(1, 5)].iter().cloned().collect(),
    });

    frames.push(DataFrame {
        metric: "Sunday",
        data: [(1, 14)].iter().cloned().collect(),
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
    //   "title": {"text": "Radar Chart Demo"},
    //   "tooltip": {"valueFormatter": (value) => "$value units"}
    // };

    // let chart = RadarChart::new(Default::default());
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
    //       row[i] = rand(5, 20);
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
    //       row[2] = rand(5, 20);
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
    //     let values = <Object>["New"];
    //     for (let i = 1; i < table.columns.length; i++) {
    //       values.add(rand(5, 20));
    //     }
    //     table.rows.insert(2, values);
    //   } else {
    //     table.rows.removeAt(2);
    //   }
    //   insertRow = !insertRow;
    //   chart.update();
    // });
}
