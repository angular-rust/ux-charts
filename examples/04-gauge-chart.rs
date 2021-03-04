#![allow(unused_imports)]

use ux_primitives::datatable::*;
use ux_charts::*;

fn main() {
    //   let changeDataButton = ButtonElement()..text = "Change data";
    //   document.body.append(changeDataButton);

    //   let insertRemoveRowButton = ButtonElement()..text = "Insert/remove data row";
    //   document.body.append(insertRemoveRowButton);

    //   let container = createContainer();
    //   let table = DataTable([
    //     ["Browser", "Share"],
    //     ["Memory", 25],
    // //    ["CPU", 75],
    // //    ["Disk", 40]
    //   ]);
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
