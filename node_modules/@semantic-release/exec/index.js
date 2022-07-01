const {isNil} = require('lodash');
const parseJson = require('parse-json');
const debug = require('debug')('semantic-release:exec');
const SemanticReleaseError = require('@semantic-release/error');
const exec = require('./lib/exec');
const verifyConfig = require('./lib/verify-config');

async function verifyConditions(pluginConfig, context) {
  if (!isNil(pluginConfig.verifyConditionsCmd) || !isNil(pluginConfig.cmd)) {
    verifyConfig('verifyConditionsCmd', pluginConfig);

    try {
      await exec('verifyConditionsCmd', pluginConfig, context);
    } catch (error) {
      throw new SemanticReleaseError(error.stdout, 'EVERIFYCONDITIONS');
    }
  }
}

async function analyzeCommits(pluginConfig, context) {
  if (!isNil(pluginConfig.analyzeCommitsCmd) || !isNil(pluginConfig.cmd)) {
    verifyConfig('analyzeCommitsCmd', pluginConfig);

    const stdout = await exec('analyzeCommitsCmd', pluginConfig, context);
    return stdout || undefined;
  }
}

async function verifyRelease(pluginConfig, context) {
  if (!isNil(pluginConfig.verifyReleaseCmd) || !isNil(pluginConfig.cmd)) {
    verifyConfig('verifyReleaseCmd', pluginConfig);

    try {
      await exec('verifyReleaseCmd', pluginConfig, context);
    } catch (error) {
      throw new SemanticReleaseError(error.stdout, 'EVERIFYRELEASE');
    }
  }
}

async function generateNotes(pluginConfig, context) {
  if (!isNil(pluginConfig.generateNotesCmd) || !isNil(pluginConfig.cmd)) {
    verifyConfig('generateNotesCmd', pluginConfig);

    const stdout = await exec('generateNotesCmd', pluginConfig, context);
    return stdout;
  }
}

async function prepare(pluginConfig, context) {
  if (!isNil(pluginConfig.prepareCmd) || !isNil(pluginConfig.cmd)) {
    verifyConfig('prepareCmd', pluginConfig);

    await exec('prepareCmd', pluginConfig, context);
  }
}

async function publish(pluginConfig, context) {
  if (!isNil(pluginConfig.publishCmd) || !isNil(pluginConfig.cmd)) {
    verifyConfig('publishCmd', pluginConfig);

    const stdout = await exec('publishCmd', pluginConfig, context);

    try {
      return stdout ? parseJson(stdout) : undefined;
    } catch (error) {
      debug(stdout);
      debug(error);

      debug(
        `The command ${pluginConfig.publishCmd ||
          pluginConfig.cmd} wrote invalid JSON to stdout. The stdout content will be ignored.`
      );
    }

    return undefined;
  }

  return false;
}

async function addChannel(pluginConfig, context) {
  if (!isNil(pluginConfig.addChannelCmd) || !isNil(pluginConfig.cmd)) {
    verifyConfig('addChannelCmd', pluginConfig);

    const stdout = await exec('addChannelCmd', pluginConfig, context);

    try {
      return stdout ? parseJson(stdout) : undefined;
    } catch (error) {
      debug(stdout);
      debug(error);

      debug(`The command ${pluginConfig.cmd} wrote invalid JSON to stdout. The stdout content will be ignored.`);

      return undefined;
    }
  }

  return false;
}

async function success(pluginConfig, context) {
  if (!isNil(pluginConfig.successCmd) || !isNil(pluginConfig.cmd)) {
    verifyConfig('successCmd', pluginConfig);

    await exec('successCmd', pluginConfig, context);
  }
}

async function fail(pluginConfig, context) {
  if (!isNil(pluginConfig.failCmd) || !isNil(pluginConfig.cmd)) {
    verifyConfig('failCmd', pluginConfig);

    await exec('failCmd', pluginConfig, context);
  }
}

module.exports = {
  verifyConditions,
  analyzeCommits,
  verifyRelease,
  generateNotes,
  prepare,
  publish,
  addChannel,
  success,
  fail,
};
