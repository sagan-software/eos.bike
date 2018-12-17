const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const WebpackMd5Hash = require("webpack-md5-hash");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const CompressionPlugin = require("compression-webpack-plugin");
const webpack = require("webpack");

function requireEnvVar(name) {
  const value = process.env[name];
  if (value) {
    return value;
  } else {
    console.error(name, "environment variable must be set. Exiting...");
    process.exit(1);
  }
}

function requireEnvVarPath(name) {
  return path.resolve(requireEnvVar(name));
}

const NODE_ENV = requireEnvVar("NODE_ENV");
const DIST_DIR = requireEnvVarPath("WEB_CLIENT_DIST_DIR");
const STATIC_DIR = path.join(__dirname, "static");

console.dir({
  NODE_ENV,
  DIST_DIR,
  STATIC_DIR
});

const IS_PROD = NODE_ENV === "production";

module.exports = {
  mode: NODE_ENV,
  entry: {
    index: path.join(STATIC_DIR, "index.js")
  },
  output: {
    path: path.resolve(DIST_DIR),
    filename: "[name].js"
  },
  module: {
    rules: [
      {
        test: /\.css$/,
        use: IS_PROD
          ? [
              "style-loader",
              MiniCssExtractPlugin.loader,
              "css-loader",
              "postcss-loader"
            ]
          : ["style-loader", "css-loader"]
      }
    ]
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: path.join(STATIC_DIR, "index.html"),
      hash: true,
      filename: "index.html"
    }),
    ...(IS_PROD
      ? [
          new MiniCssExtractPlugin({
            filename: "index.css"
          }),
          new WebpackMd5Hash(),
          new CompressionPlugin({
            test: /\.(html|css|js|wasm)$/
          })
        ]
      : [new webpack.HotModuleReplacementPlugin()])
  ],
  devServer: {
    https: true,
    historyApiFallback: true,
    hot: true
  }
};
