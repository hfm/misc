module.exports = { hello };

function hello(context, events, next) {
  console.log(`Hello Func ${context.vars.target}`);
  next();
}
