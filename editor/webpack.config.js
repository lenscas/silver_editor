const path = require("path");

module.exports = {
  mode: "development",
  devtool: "source-maps",
  entry: path.join(__dirname, "/js/app.ts"),
  output: {
    filename: "app.js",
    path: __dirname,
    libraryTarget: "window",
    library: "silver_editor",
  },
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        loader: "ts-loader",
        exclude: /node_modules/,
      },
    ],
  },
  resolve: {
    extensions: [".tsx", ".ts", ".js"],
  },
};
