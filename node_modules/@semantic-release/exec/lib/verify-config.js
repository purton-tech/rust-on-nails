const {isNil, isString} = require('lodash');
const AggregateError = require('aggregate-error');
const getError = require('./get-error');

const isNonEmptyString = value => isString(value) && value.trim();
const isOptional = validator => value => isNil(value) || validator(value);

const VALIDATORS = {
  cmd: isNonEmptyString,
  shell: isOptional(shell => shell === true || isNonEmptyString(shell)),
  execCwd: isOptional(isNonEmptyString),
};

module.exports = (cmdProp, {shell, execCwd, ...pluginConfig}) => {
  const cmd = pluginConfig[cmdProp] ? cmdProp : pluginConfig.cmd ? 'cmd' : cmdProp;

  const errors = Object.entries({shell, execCwd, cmd: pluginConfig[cmd]}).reduce(
    (errors, [option, value]) =>
      VALIDATORS[option](value)
        ? errors
        : [...errors, getError(`EINVALID${option.toUpperCase()}`, {[option]: value, cmdProp})],
    []
  );

  if (errors.length > 0) {
    throw new AggregateError(errors);
  }
};
