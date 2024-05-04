import test from "ava";

import { readFileToMd5 } from "../index.js";

test("readFileToMd5 from native", (t) => {
  t.is(
    readFileToMd5(
      "d:/新建文件夹/1493492475-1-192 - 副本 - 副本 - 副本 - 副本 - 副本.mp4"
    ),
    "815d9e44e59a9ed027a8b38b2cff2fe8"
  );
});
