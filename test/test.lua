local log = function(...)
  redis.log(redis.LOG_NOTICE, ...)
end

function xpendclaim(keys, args)
  log("xxx", #keys, keys[1])
  return 1
end
