const express = require("express");
const path = require("path");

const app = express();
const port = 3000;

const staticDir = path.join(__dirname, "public");
app.use(express.static(staticDir));

const flintDir = "/flint";
const flintPath = path.join(__dirname, "..", "dist");
app.use(flintDir, express.static(flintPath));

app.listen(port, () => {
  console.log(`Server is running at http://localhost:${port}`);
});
