#![allow(unused_imports)]

use ux_primitives::datatable::*;
use ux_charts::*;

fn main() {
    // let table = DataTable([
    //   ["Categories", "Series 1"],
    //   ["Monday", 8],
    //   ["Tuesday", 17],
    //   ["Wednesday", 7],
    //   ["Thursday", 16],
    //   ["Friday", 12],
    //   ["Saturday", 5],
    //   ["Sunday", 14]
    // ]);

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
