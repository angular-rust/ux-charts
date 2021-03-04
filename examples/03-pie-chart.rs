#![allow(unused_imports)]

use ux_primitives::datatable::*;
use ux_charts::*;

fn main() {
    // let changeDataButton = ButtonElement()..text = "Change data";
    // document.body.append(changeDataButton);

    // let insertRemoveRowButton = ButtonElement()..text = "Insert/remove data row";
    // document.body.append(insertRemoveRowButton);

    // let container = createContainer();
    // let table = DataTable([
    //   ["Browser", "Share"],
    //   ["Chrome", 35],
    //   ["Firefox", 20],
    //   ["IE", 30],
    //   ["Opera", 5],
    //   ["Safari", 8],
    //   ["Other", 2]
    // ]);

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
