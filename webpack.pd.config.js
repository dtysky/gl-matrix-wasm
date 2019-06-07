#!/usr/bin/env node
/**
 * @File   : webpack.pd.confg.js
 * @Author : dtysky (dtysky@outlook.com)
 * @Date   : 2018-6-8 16:18:56
 * @Description:
 */
const webpack = require('webpack');
const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const CleanWebpackPlugin = require('clean-webpack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');

const outPath = path.resolve(__dirname, 'dist');

module.exports = {
  entry: {
    main: path.resolve(__dirname, './demo/index.tsx'),
    'react-packet': ['react', 'react-dom', 'gl-matrix']
  },

  output: {
    path: outPath,
    filename: '[name].[hash].js',
    webassemblyModuleFilename: '[modulehash].wasm',
    publicPath: './'
  },

  resolve: {
    extensions: ['.ts', '.tsx', '.js', '.md', '.wasm']
  },

  mode: 'production',

  externals: {
    'fs': true,
    'path': true
  },
  
  module: {
    rules: [
      {
        enforce: 'pre',
        test: /\.tsx?$/,
        use: [
          {
            loader: 'awesome-typescript-loader'
          }
        ],
        exclude: /node_modules/
      },
      {
        test: /\.wasm$/,
        type: 'webassembly/experimental'
      },
      {
        test: /\.(css|scss)$/,
        use: [
          {
            loader: 'style-loader'
          },
          {
            loader: 'css-loader'
          },
          {
            loader: 'postcss-loader'
          },
          {
            loader: 'sass-loader'
          }
        ]
      },
      {
        test: /\.md$/,
        use: [
          {
            loader: 'raw-loader'
          }
        ]
      },
      {
        test: /\.(png|jpg|gif|svg|mp4)$/,
        use: {
          loader: 'url-loader',
          query: {
            limit: 15000
          }
        }
      }
    ]
  },

  optimization: {
    splitChunks: {
      minChunks: 2,
      cacheGroups: {                 // 这里开始设置缓存的 chunks
        'react-packet': {                   // key 为entry中定义的 入口名称
          chunks: 'initial',        // 必须三选一： "initial" | "all" | "async"(默认就是异步)
          test: /react/,     // 正则规则验证，如果符合就提取 chunk
          name: 'react-packet',           // 要缓存的 分隔出来的 chunk 名称
          enforce: true
        }
      }
    }
  },

  plugins: [
    new CleanWebpackPlugin(
      ['*'],
      {root: outPath}
    ),
    new CopyWebpackPlugin(
      [
        {
          from: path.resolve(__dirname, './demo/assets'),
          to: path.resolve(__dirname, './dist/assets')
        }
      ]
    ),
    new webpack.DefinePlugin({
      NODE_ENV: JSON.stringify('production')
    }),
    new HtmlWebpackPlugin({
      template: './demo/index.html'
    })
  ]
};
