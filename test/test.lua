local log = function(...)
  local li = {}
  for _, v in ipairs({ ... }) do
    table.insert(li, cjson.encode(v))
  end
  redis.log(redis.LOG_NOTICE, unpack(li))
end

local XPENDING = function(stream, group, idle, limit)
  return redis.call("XPENDING", stream, group, "IDLE", idle, "-", "+", limit)
end

local xclaim = function(stream, group, customer, min_idle, ...)
  return redis.call("XCLAIM", stream, group, customer, min_idle, ...)
end

function xpendclaim(keys, args)
  local stream, group, customer = unpack(keys)
  local idle, limit = unpack(args)
  idle = tonumber(idle)
  local li = XPENDING(stream, group, idle, limit)
  if #li > 0 then
    local id_li = {}
    local id_retry = {}
    for _, v in ipairs(li) do
      local id = v[1]
      table.insert(id_li, id)
      id_retry[id] = v[4]
    end
    local r = {}
    for _, v in ipairs(xclaim(stream, group, customer, idle, unpack(id_li))) do
      local id, msg = unpack(v)
      table.insert(r, { id_retry[id], id, msg[2] })
    end
    return cjson.encode(r)
  else
    return "[]"
  end
end
