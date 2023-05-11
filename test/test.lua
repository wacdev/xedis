local log = function(...)
  redis.log(redis.LOG_NOTICE, ...)
end

function xpendclaim(keys, args)
  log(keys, args)
end
