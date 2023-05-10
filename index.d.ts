/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export function conn(server: Server, username: OptionString, password: OptionString, database?: number | undefined | null): Promise<Xedis>
export class Xedis {
  del(key: Array<Bin> | Bin): Promise<number>
  exist(key: Array<Bin> | Bin): Promise<number>
  zrem(key: Bin, key: Array<Bin> | Bin): Promise<number>
  expire(key: Bin, ex: number): Promise<boolean>
  get(key: Bin): Promise<OptionString>
  getB(key: Bin): Promise<Val>
  hdel(map: Bin, key: Bin): Promise<number>
  hexist(map: Bin, key: Bin): Promise<boolean>
  hget(map: Bin, key: Bin): Promise<OptionString>
  hgetB(map: Bin, key: Bin): Promise<Val>
  hincrby(map: Bin, key: Bin, val: number): Promise<number>
  hmget(map: Bin, li: Array<Bin>): Promise<Array<OptionString>>
  hmgetB(map: Bin, li: Array<Bin>): Promise<Array<Val>>
  quit(): Promise<void>
  sadd(set: Bin, val: Bin): Promise<number>
  smembers(set: Bin): Promise<Array<Val>>
  zscore(zset: Bin, key: Bin): Promise<number | null>
  fcall(name: Bin, key: Array<Bin>, val: Array<Bin>): Promise<void>
  fcallR(name: Bin, key: Array<Bin>, val: Array<Bin>): Promise<void>
  fbool(name: Bin, key: Array<Bin>, val: Array<Bin>): Promise<boolean | null>
  fboolR(name: Bin, key: Array<Bin>, val: Array<Bin>): Promise<boolean | null>
  fbin(name: Bin, key: Array<Bin>, val: Array<Bin>): Promise<Val | null>
  fbinR(name: Bin, key: Array<Bin>, val: Array<Bin>): Promise<Val | null>
  fnum(name: Bin, key: Array<Bin>, val: Array<Bin>): Promise<number | null>
  fnumR(name: Bin, key: Array<Bin>, val: Array<Bin>): Promise<number | null>
  fstr(name: Bin, key: Array<Bin>, val: Array<Bin>): Promise<string | null>
  fstrR(name: Bin, key: Array<Bin>, val: Array<Bin>): Promise<string | null>
  setex(key: Bin, val: Bin, ex: number): Promise<void>
  fnload(code: Bin): Promise<string>
  hincr(map: Bin, key: Bin): Promise<number>
  zincrby(zset: Bin, key: Bin, score: number): Promise<number>
  zincr(zset: Bin, key: Bin): Promise<number>
  set(key: Bin, val: Bin): Promise<void>
  zadd(zset: Bin, key: Record<string, number> | Array<[Bin, number]> | Bin, score?: number | undefined | null): Promise<number>
  zaddXx(zset: Bin, key: Record<string, number> | Array<[Bin, number]> | Bin, score?: number | undefined | null): Promise<number>
  zaddNx(zset: Bin, key: Record<string, number> | Array<[Bin, number]> | Bin, score?: number | undefined | null): Promise<number>
  hset(map: Bin, key: BinOrMap, val?: Bin | undefined | null): Promise<void>
  zrangebyscoreWithscores(zset: Bin, opt?: Record<string, StrOrN> | undefined | null): Promise<Array<[Val, number]>>
  zrangebyscore(zset: Bin, opt?: Record<string, StrOrN> | undefined | null): Promise<Array<Val>>
  zrevrangebyscoreWithscore(zset: Bin, opt?: Record<string, StrOrN> | undefined | null): Promise<Array<[Val, number]>>
  zrevrangebyscore(zset: Bin, opt?: Record<string, StrOrN> | undefined | null): Promise<Array<Val>>
}
export class Server {
  static cluster(hostPortLi: Array<[string, number]>): Server
  static hostPort(host: string, port: number): Server
}
