#![allow(unused_imports)]
#![allow(unused_variables)]

use ux_charts::*;
use ux_primitives::datatable::*;

fn main() {
    // let changeDataButton = ButtonElement()..text = "Change data";
    // document.body.append(changeDataButton);

    // let insertRemoveRowButton = ButtonElement()..text = "Insert/remove data row";
    // document.body.append(insertRemoveRowButton);

    // let container = createContainer();

    let metadata = vec![
        Channel {
            name: "Browser",
            tag: 0,
            visible: true,
        },
        Channel {
            name: "Share",
            tag: 1,
            visible: true,
        },
    ];

    let mut frames = vec![DataFrame {
        metric: "Chrome",
        data: [(1, 35)].iter().cloned().collect(),
    }];

    frames.push(DataFrame {
        metric: "Firefox",
        data: [(1, 20)].iter().cloned().collect(),
    });

    frames.push(DataFrame {
        metric: "IE",
        data: [(1, 30)].iter().cloned().collect(),
    });

    frames.push(DataFrame {
        metric: "Opera",
        data: [(1, 5)].iter().cloned().collect(),
    });

    frames.push(DataFrame {
        metric: "Safari",
        data: [(1, 8)].iter().cloned().collect(),
    });

    frames.push(DataFrame {
        metric: "Other",
        data: [(1, 2)].iter().cloned().collect(),
    });

    let stream = DataStream::new(metadata, frames);

    // let chart = PieChart::new(Default::default());
    // chart.draw(table, {
    //   "animation": {
    //     "onEnd": () {
    //       changeDataButton.disabled = false;
    //       insertRemoveRowButton.disabled = false;
    //     }
    //   },
    //   "pieHole": .5,
    //   "series": {
    //     "counterclockwise": true,
    //     "labels": {"enabled": true},
    //     "startAngle": 90 + 10 * 360,
    //   },
    //   "title": {"text": "Pie Chart Demo"},
    // });

    // fn disableAllButtons() {
    //   changeDataButton.disabled = true;
    //   insertRemoveRowButton.disabled = true;
    // }

    // changeDataButton.onClick.listen((_) {
    //   disableAllButtons();
    //   for (let row in table.rows) {
    //     for (let i = 1; i < table.columns.length; i++) {
    //       row[i] = rand(2, 25);
    //     }
    //   }
    //   chart.update();
    // });

    // let insertRow = true;
    // insertRemoveRowButton.onClick.listen((_) {
    //   insertRemoveRowButton.disabled = true;
    //   if (insertRow) {
    //     let values = ["New", 6];
    //     table.rows.insert(2, values);
    //   } else {
    //     table.rows.removeAt(2);
    //   }
    //   insertRow = !insertRow;
    //   chart.update();
    // });
}
