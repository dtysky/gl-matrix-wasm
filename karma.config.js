#!/usr/bin/env node
/**
 * @File   : karma.conf..js
 * @Author : dtysky (dtysky@outlook.com)
 * @Link   : dtysky.moe
 * @Date   : 2019/6/7 下午9:41:32
 */
const path = require('path');
const outPath = path.resolve(__dirname, 'dist');

module.exports = function(config) {
  config.set({

    basePath: '',

    frameworks: ['mocha'],

    files: [
      // 'spec/**/*-spec.ts'
      'spec/mat3-spec.ts'
    ],

    port: 9876,

    logLevel: config.LOG_INFO,

    colors: true,

    autoWatch: true,

    browsers: ['Chrome'],

    webpack: {
      // karma watches the test entry points
      // (you don't need to specify the entry option)
      // webpack watches dependencies

      // webpack configuration
      output: {
        path: outPath,
        filename: '[name].[hash].js',
        webassemblyModuleFilename: '[modulehash].wasm',
        publicPath: './'
      },
    
      resolve: {
        extensions: ['.ts', '.tsx', '.js', '.md', '.wasm']
      },
    
      mode: 'development',

      module: {
        rules: [
          {
            enforce: 'pre',
            test: /\.(ts|js)$/,
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
          }
        ]
      },

      optimization: {
        splitChunks: {
          minChunks: 2
        }
      },
    },

    preprocessors: {
      'spec/**/*.ts': ['webpack']
    },

    // reporters: ['progress'],

    karmaTypescriptConfig: {
      reports: {
        'html': 'reports/coverage'
      }
    },

    // singleRun: true
  })
};
