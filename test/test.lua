local log = function(...)
  local li = {}
  for _, v in ipairs({ ... }) do
    table.insert(li, cjson.encode(v))
  end
  redis.log(redis.LOG_NOTICE, unpack(li))
end

local XPENDING = function(key, group, idle, limit)
  return redis.call("XPENDING", key, group, "IDLE", idle, "-", "+", limit)
end

local xclaim = function(key, group, customer, min_idle, ...)
  return redis.call("XCLAIM", key, group, consumer, min_idle, ...)
end

function xpendclaim(keys, args)
  local key, group, customer = unpack(keys)
  local idle, limit = unpack(args)
  local li = XPENDING(key, group, idle, limit)
  log(li)
  local id_li = {}
  for _, v in ipairs(li) do
    -- id_li.insert(v[1])
  end
  -- LOG(unpack(id_li))
  -- xclaim(key, group, customer, idle, )
  return cjson.encode(li)
end
