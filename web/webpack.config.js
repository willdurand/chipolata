const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require("path");

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "..", "docs"),
    filename: "bootstrap.js",
  },
  mode: process.env.NODE_ENV || "development",
  plugins: [new CopyWebpackPlugin(["index.html", "styles.css"])],
};
