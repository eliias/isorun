import path from "path";
import {fileURLToPath} from 'url';
import webpack from "webpack";
import {VueLoaderPlugin} from "vue-loader";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

export default {
    mode: "development",
    devtool: "source-map",
    entry: {
        app: "./app/javascript/app.ts",
        sidebar: "./app/javascript/sidebar.ts"
    },
    module: {
        rules: [{
            test: /\.tsx?$/, use: [{
                loader: 'ts-loader', options: {
                    appendTsSuffixTo: [/\.vue$/],
                },
            }], exclude: /node_modules/,
        }, {
            test: /\.vue$/, loader: 'vue-loader', options: {
                reactivityTransform: true
            }
        }]
    }, output: {
        filename: "[name].js", sourceMapFilename: "[file].map", path: path.resolve(__dirname, "app/assets/builds"),
    }, resolve: {
        extensions: ['.ts', '.tsx']
    }, plugins: [new webpack.optimize.LimitChunkCountPlugin({
        maxChunks: 1
    }), new webpack.DefinePlugin({
        __VUE_OPTIONS_API__: false, __VUE_PROD_DEVTOOLS__: false,
    }), new VueLoaderPlugin()]
}
