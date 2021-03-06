import { Injectable } from '@angular/core'
import { HttpClient } from '@angular/common/http'
import { Observable } from 'rxjs/Observable'

import { environment } from '../../environments/environment'
import {
  Customer,
  Goods,
  JoinedGoods,
  JoinedPurchase,
  JoinedReceipt,
  JoinedStaff,
  Purchase,
  Receipt,
  Staff,
  User
} from '../shared'

@Injectable()
export class ApiService {

  private customerUrl = environment.api_url + 'customer'
  private goodsUrl = environment.api_url + 'goods'
  private staffUrl = environment.api_url + 'staff'
  private receiptUrl = environment.api_url + 'receipt'
  private purchaseUrl = environment.api_url + 'purchase'
  private loginUrl = environment.api_url + 'login'

  constructor (private http: HttpClient) {}

  getCustomers (): Observable<Customer[]> {
    return this.http.get<Customer[]>(this.customerUrl)
  }

  updateCustomer (customer: Customer): Observable<any> {
    return this.http.put(this.customerUrl, customer)
  }

  createCustomer (customer: Customer): Observable<any> {
    return this.http.post(this.customerUrl, customer)
  }

  deleteCustomer (id: number): Observable<any> {
    return this.http.request('DELETE', this.customerUrl, {body: {id}})
  }

  getGoods (): Observable<JoinedGoods[]> {
    return this.http.get<JoinedGoods[]>(this.goodsUrl)
  }

  updateGoods (goods: Goods): Observable<any> {
    return this.http.put(this.goodsUrl, goods)
  }

  createGoods (goods: Goods): Observable<any> {
    return this.http.post(this.goodsUrl, goods)
  }

  deleteGoods (id: number): Observable<any> {
    return this.http.request('DELETE', this.goodsUrl, {body: {id}})
  }

  getStaffs (): Observable<JoinedStaff[]> {
    return this.http.get<JoinedStaff[]>(this.staffUrl)
  }

  updateStaff (staff: Staff): Observable<any> {
    return this.http.put(this.staffUrl, staff)
  }

  createStaff (staff: Staff): Observable<any> {
    return this.http.post(this.staffUrl, staff)
  }

  deleteStaff (id: number): Observable<any> {
    return this.http.request('DELETE', this.staffUrl, {body: {id}})
  }

  getPurchases (): Observable<JoinedPurchase[]> {
    return this.http.get<JoinedPurchase[]>(this.purchaseUrl)
  }

  createPurchase (purchase: Purchase): Observable<any> {
    return this.http.post(this.purchaseUrl, purchase)
  }

  getReceipts (): Observable<JoinedReceipt[]> {
    return this.http.get<JoinedReceipt[]>(this.receiptUrl)
  }

  createReceipt (receipt: Receipt): Observable<any> {
    return this.http.post(this.receiptUrl, receipt)
  }

  login (username: string, password: string): Observable<User> {
    return this.http.post<User>(this.loginUrl, {username, password})
  }

}
