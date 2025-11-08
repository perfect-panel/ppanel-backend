// @ts-ignore
/* eslint-disable */
import request from '@/utils/request';

/** Get subscribe list GET /v1/public/subscribe/list */
export async function querySubscribeList(
  // 叠加生成的Param类型 (非body参数swagger默认没有生成对象)
  params: API.QuerySubscribeListParams,
  options?: { [key: string]: any },
) {
  return request<API.Response & { data?: API.QuerySubscribeListResponse }>(
    '/v1/public/subscribe/list',
    {
      method: 'GET',
      params: {
        ...params,
      },
      ...(options || {}),
    },
  );
}

/** Get user subscribe node info GET /v1/public/subscribe/node/list */
export async function queryUserSubscribeNodeList(options?: { [key: string]: any }) {
  return request<API.Response & { data?: API.QueryUserSubscribeNodeListResponse }>(
    '/v1/public/subscribe/node/list',
    {
      method: 'GET',
      ...(options || {}),
    },
  );
}

/** Update user subscribe note PUT /v1/public/user/subscribe_note */
export async function updateUserSubscribeNote(
  body: API.UpdateUserSubscribeNoteRequest,
  options?: { [key: string]: any },
) {
  return request<API.Response>('/v1/public/user/subscribe_note', {
    method: 'PUT',
    headers: {
      'Content-Type': 'application/json',
    },
    data: body,
    ...(options || {}),
  });
}
