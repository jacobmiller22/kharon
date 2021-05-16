const express = require("express");
const app = express();
const PORT = process.env.PORT || 5000;

app.get("/", (req, res) => {
  // console.log(`PORT: ${PORT}, received a req`);
  res.status(200).json({ response: "hello world" });
});

app.listen(PORT, (err) => {
  if (err) {
    return console.log(`ERROR on PORT: ${PORT}: `, err);
  }
  return console.log(`Express Server Listening on PORT: ${PORT}`);
});
