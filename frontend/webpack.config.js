const path = require('path');

const VueLoaderPlugin = require('vue-loader/lib/plugin');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');

module.exports = {
  entry: {
    app: './src/app/app-client.js',
    index: './src/pages/index.js',
  },

  output: {
    path: path.resolve(__dirname, 'static/'),  // TODO: dist
    publicPath: '/static/',
    // assetModuleFilename: '../img/[hash][ext][query]',
    assetModuleFilename: 'img/[name][ext]',
    filename: 'js/[name].js'
  },

  devServer: {
    contentBase: false,
    // contentBase: path.join(__dirname, 'static'),
    // contentBasePublicPath: '/static',
    watchOptions: {
      // poll: 1000,
    },
    inline: true,

    host: '0.0.0.0',
    port: 9000,
    // sockHost: 'localhost',
    sockPort: 8080, // TODO: env variable
    disableHostCheck: true,

    compress: true,
    headers: {
      'Access-Control-Allow-Origin': '*',
      'Access-Control-Allow-Headers': '*',
      'Access-Control-Allow-Methods': '*',
    }
  },

  mode: 'development',
  // stats: 'verbose',

  module: {
    rules: [
      {
        test: /\.vue$/,
        loader: 'vue-loader'
      },
      {
        test: /\.js$/,
        loader: 'babel-loader'
      },
      {
        test: /\.css$/,
        sideEffects: true,
        use: [
          MiniCssExtractPlugin.loader,
          'css-loader'
        ]
      },
      {
        test: /\.scss$/,
        sideEffects: true,
        use: [
          MiniCssExtractPlugin.loader,
          'css-loader',
          {
            loader: 'postcss-loader',
            options: {
              postcssOptions: {
                plugins: function () {
                  return [
                    require('autoprefixer')
                  ];
                }
              }
            }
          },
          'resolve-url-loader', // официальный костыль для того, чтобы работали относительные урлы в sass
          {
            loader: 'sass-loader',
            options: {
              sourceMap: true,
            }
          }
        ]
      },
      {
        test: /\.(png|svg|jpe?g|gif)$/i,
        type: 'asset/resource',
      },
    ]
  },

  plugins: [
    new VueLoaderPlugin(),
    new MiniCssExtractPlugin({
      filename: "css/[name].css"
    })
  ]
}
