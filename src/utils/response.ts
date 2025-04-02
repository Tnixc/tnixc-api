import { StatusCodes } from "http-status-codes";
import type { StatusCodes as StatusCodeType } from "http-status-codes";

type Response = {
  status: boolean;
  message: string;
  code: StatusCodeType;
  data?: any;
  error?: any;
};

export async function okResponse(
  data: any,
  message: string,
  code: StatusCodeType = StatusCodes.OK,
) {
  let res: Response = {
    code,
    status: true,
    message,
  };

  res.data = data;

  return res;
}

export async function errorResponse(
  message: string,
  error: any,
  code: StatusCodeType = StatusCodes.BAD_REQUEST,
) {
  let res: Response = {
    code,
    status: false,
    message,
  };

  res.error = error;

  return res;
}

export async function notFoundResponse(
  message: string = "Data tidak ditemukan",
  code: StatusCodeType = StatusCodes.NOT_FOUND,
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
  message: string = "Maaf, terjadi kesalahan pada server",
  error: any,
  code: StatusCodeType = StatusCodes.INTERNAL_SERVER_ERROR,
) {
  let res: Response = {
    code,
    status: false,
    message,
  };

  res.error = error;

  return res;
}
