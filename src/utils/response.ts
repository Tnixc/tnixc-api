import { StatusCodes } from 'http-status-codes';
import type { StatusCodes as StatusCodeType } from 'http-status-codes';
import _ from 'lodash';

type Response = {
  status: boolean;
  message: string;
  code: StatusCodeType;
  data?: any;
  error?: any;
};

export async function okResponse(
  data: any,
  message: string = 'Berhasil medapatkan data',
  code: StatusCodeType = StatusCodes.OK
) {
  let res: Response = {
    code,
    status: true,
    message,
  };

  if (!_.isEmpty(data)) {
    res.data = data;
  }

  return res;
}

export async function errorResponse(
  message: string = 'Gagal mendapatkan data',
  error: any,
  code: StatusCodeType = StatusCodes.BAD_REQUEST
) {
  let res: Response = {
    code,
    status: false,
    message,
  };

  if (!_.isEmpty(error)) {
    res.error = error;
  }

  return res;
}

export async function notFoundResponse(
  message: string = 'Data tidak ditemukan',
  code: StatusCodeType = StatusCodes.NOT_FOUND
) {
  let res: Response = {
    code,
    status: false,
    message,
    data: null,
  };

  return res;
}

export async function internalServerErrorResponse(
  message: string = 'Maaf, terjadi kesalahan pada server',
  error: any,
  code: StatusCodeType = StatusCodes.INTERNAL_SERVER_ERROR
) {
  let res: Response = {
    code,
    status: false,
    message,
  };

  if (!_.isEmpty(error)) {
    res.error = error;
  }

  return res;
}
