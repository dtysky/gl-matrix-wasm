#!/usr/bin/env node
/**
 * @File   : karma.conf..js
 * @Author : dtysky (dtysky@outlook.com)
 * @Link   : dtysky.moe
 * @Date   : 2019/6/7 下午9:41:32
 */
module.exports = function(config) {
  config.set({

    basePath: '',

    frameworks: ['mocha', 'karma-typescript'],

    files: [
      {pattern: 'src/**/*.ts'},
      {pattern: 'spec/**/*-spec.ts'}
    ],

    port: 9876,

    logLevel: config.LOG_INFO,

    colors: true,

    autoWatch: true,

    browsers: ['Chrome'],

    preprocessors: {
      'src/**/*.js': ['karma-typescript'],
      'spec/**/*.js': ['karma-typescript']
    },

    reporters: ['progress', 'karma-typescript'],

    karmaTypescriptConfig: {
      reports: {
        'html': 'reports/coverage'
      }
    },

    singleRun: true
  })
};
