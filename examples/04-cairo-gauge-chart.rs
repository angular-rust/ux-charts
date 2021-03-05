#![allow(unused_imports)]
#![allow(unused_variables)]

use ux_charts::*;
use ux_primitives::datatable::*;

fn main() {
    //   let changeDataButton = ButtonElement()..text = "Change data";
    //   document.body.append(changeDataButton);

    //   let insertRemoveRowButton = ButtonElement()..text = "Insert/remove data row";
    //   document.body.append(insertRemoveRowButton);

    //   let container = createContainer();

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

    let frames = vec![DataFrame {
        metric: "Memory",
        data: [(1, 25)].iter().cloned().collect(),
    }];

    // frames.push(DataFrame {
    //     metric: "CPU",
    //     data: [(1, 75)].iter().cloned().collect(),
    // });

    // frames.push(DataFrame {
    //     metric: "Disk",
    //     data: [(1, 40)].iter().cloned().collect(),
    // });

    let stream = DataStream::new(metadata, frames);

    // let chart = GaugeChart::new(Default::default());
    //   chart.draw(table, {
    //     "animation": {
    //       "easing": (f64 t) {
    //         t = 4 * t - 2;
    //         return (t * t * t - t) / 12 + .5;
    //       },
    //       "onEnd": () {
    //         changeDataButton.disabled = false;
    //         insertRemoveRowButton.disabled = false;
    //       }
    //     },
    //     "gaugeLabels": {"enabled": false},
    //     "title": {"text": "Gauge Chart Demo"},
    //   });

    //   fn disableAllButtons() {
    //     changeDataButton.disabled = true;
    //     insertRemoveRowButton.disabled = true;
    //   }

    //   changeDataButton.onClick.listen((_) {
    //     disableAllButtons();
    //     for (let row in table.rows) {
    //       for (let i = 1; i < table.columns.length; i++) {
    //         row[i] = rand(0, 101);
    //       }
    //     }
    //     chart.update();
    //   });

    //   let insertRow = true;
    //   insertRemoveRowButton.onClick.listen((_) {
    //     insertRemoveRowButton.disabled = true;
    //     if (insertRow) {
    //       let values = ["New", rand(0, 101)];
    //       table.rows.insert(1, values);
    //     } else {
    //       table.rows.removeAt(1);
    //     }
    //     insertRow = !insertRow;
    //     chart.update();
    //   });
}
