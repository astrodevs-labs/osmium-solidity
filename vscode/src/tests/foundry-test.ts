import {exec} from 'child_process';
import * as vscode from "vscode";

type TestResult = {
    status: string,
    reason: string | null,
    counterexample: any | null,
    logs: any[],
    // eslint-disable-next-line @typescript-eslint/naming-convention
    decoded_logs: string[]
    kind: any,
    traces: any
    coverage: any
    // eslint-disable-next-line @typescript-eslint/naming-convention
    labeled_addresses: {
        [key: string]: string
    }
    debug: any | null
    breakpoints: any
};

type SuiteResult = {
    duration: {
        nanos: number
        secs: number
    },
    // eslint-disable-next-line @typescript-eslint/naming-convention
    test_results: {
        [key: string]: TestResult
    },
    warnings: string[]
};

type FileResult = {
    [key: string]: SuiteResult
};

const hasForge = async (workspace: string) => {
    return new Promise((resolve, reject) => {
        exec('forge --version', {
            cwd: workspace
        }, (err, stdout, stderr) => {
            if (err) {
                console.log(err);
                vscode.window.showErrorMessage('Forge not found. Please install it and try again.');
                resolve(false);
            } else {
                resolve(true);
            }
        });
    });
};

const testAll = async (workspace: string): Promise<FileResult> => {
    return new Promise(async (resolve, reject) => {
        if (!(await hasForge(workspace))) {
            reject("No forge found");
        }

        exec('forge test --json', {
            cwd: workspace
        }, (error, stdout, stderr) => {
            if (error) { // An error is returned by node if the forge test command fails, which is the case if a test fails
              if (!stderr.length) {
                return resolve(JSON.parse(stdout));
              }
              console.log(stderr);
              vscode.window.showErrorMessage(
                "Error while running forge tests."
              );
              reject(stderr);
            } else {
                resolve(JSON.parse(stdout));
            }
        });
    });
};

const testContract = (workspace: string, contractName: string): Promise<FileResult> => {
    return new Promise(async (resolve, reject) => {
        if (!(await hasForge(workspace))) {
            reject("No forge found");
        }

        exec(`forge test --json --match-contract '${contractName}'`, {
            cwd: workspace
        }, (error, stdout, stderr) => {
            if (error) { // An error is returned by node if the forge test command fails, which is the case if a test fails
                if (!stderr.length) {
                    return resolve(JSON.parse(stdout));
                }
                console.log(stderr);
                vscode.window.showErrorMessage('Error while running forge tests.');
                reject(stderr);
            } else {
                resolve(JSON.parse(stdout));
            }
        });
    });
};

const testFunction = (workspace: string, contractName: string, functionName: string): Promise<FileResult> => {
    return new Promise(async (resolve, reject) => {
        if (!(await hasForge(workspace))) {
            reject("No forge found");
        }
        exec(`forge test --json --match-contract '${contractName}' --match-test '${functionName}'`, {
            cwd: workspace
        }, (error, stdout, stderr) => {
            if (error) { // An error is returned by node if the forge test command fails, which is the case if a test fails
                if (!stderr.length) {
                    return resolve(JSON.parse(stdout));
                }
                console.log(stderr);
                vscode.window.showErrorMessage('Error while running forge tests.');
                reject(stderr);
            } else {
                resolve(JSON.parse(stdout));
            }
        });
    });
};


export {hasForge, testAll, testContract, testFunction, FileResult, SuiteResult, TestResult};