import { Injectable } from '@angular/core'
import { HttpClient } from '@angular/common/http'
import { Observable } from 'rxjs/Observable'

import { environment } from '../../environments/environment'
import { Customer, JoinedGoods, JoinedStaff, Staff } from '../shared'

@Injectable()
export class ApiService {

  private customerUrl = environment.api_url + 'customer'
  private goodsUrl = environment.api_url + 'goods'
  private staffUrl = environment.api_url + 'staff'

  constructor (private http: HttpClient) {}

  getCustomers (): Observable<Customer[]> {
    return this.http.get<Customer[]>(this.customerUrl)
  }

  updateCustomer (customer: Customer): Observable<any> {
    return this.http.put(this.customerUrl, customer)
  }

  getGoods (): Observable<JoinedGoods[]> {
    return this.http.get<JoinedGoods[]>(this.goodsUrl)
  }

  getStaffs (): Observable<JoinedStaff[]> {
    return this.http.get<JoinedStaff[]>(this.staffUrl)
  }

  updateStaff (staff: Staff): Observable<any> {
    return this.http.put(this.staffUrl, staff)
  }

}