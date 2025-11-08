declare namespace API {
  type Ads = {
    id: number;
    title: string;
    type: string;
    content: string;
    description: string;
    target_url: string;
    start_time: number;
    end_time: number;
    status: number;
    created_at: number;
    updated_at: number;
  };

  type AlipayNotifyResponse = {
    return_code: string;
  };

  type Announcement = {
    id: number;
    title: string;
    content: string;
    show: boolean;
    pinned: boolean;
    popup: boolean;
    created_at: number;
    updated_at: number;
  };

  type AnyTLS = {
    port: number;
    security_config: SecurityConfig;
  };

  type Application = {
    id: number;
    icon: string;
    name: string;
    description: string;
    subscribe_type: string;
  };

  type ApplicationPlatform = {
    ios?: ApplicationVersion[];
    macos?: ApplicationVersion[];
    linux?: ApplicationVersion[];
    android?: ApplicationVersion[];
    windows?: ApplicationVersion[];
    harmony?: ApplicationVersion[];
  };

  type ApplicationResponse = {
    applications: ApplicationResponseInfo[];
  };

  type ApplicationResponseInfo = {
    id: number;
    name: string;
    icon: string;
    description: string;
    subscribe_type: string;
    platform: ApplicationPlatform;
  };

  type ApplicationVersion = {
    id: number;
    url: string;
    version: string;
    description: string;
    is_default: boolean;
  };

  type AppUserSubcbribe = {
    id: number;
    name: string;
    upload: number;
    traffic: number;
    download: number;
    device_limit: number;
    start_time: string;
    expire_time: string;
    list: AppUserSubscbribeNode[];
  };

  type AppUserSubscbribeNode = {
    id: number;
    name: string;
    uuid: string;
    protocol: string;
    relay_mode: string;
    relay_node: string;
    server_addr: string;
    speed_limit: number;
    tags: string[];
    traffic: number;
    traffic_ratio: number;
    upload: number;
    config: string;
    country: string;
    city: string;
    latitude: string;
    longitude: string;
    created_at: number;
    download: number;
  };

  type AuthConfig = {
    mobile: MobileAuthenticateConfig;
    email: EmailAuthticateConfig;
    device: DeviceAuthticateConfig;
    register: PubilcRegisterConfig;
  };

  type AuthMethodConfig = {
    id: number;
    method: string;
    config: Record<string, any>;
    enabled: boolean;
  };

  type BalanceLog = {
    type: number;
    user_id: number;
    amount: number;
    order_no?: string;
    balance: number;
    timestamp: number;
  };

  type BatchDeleteCouponRequest = {
    ids: number[];
  };

  type BatchDeleteDocumentRequest = {
    ids: number[];
  };

  type BatchDeleteSubscribeGroupRequest = {
    ids: number[];
  };

  type BatchDeleteSubscribeRequest = {
    ids: number[];
  };

  type BatchDeleteUserRequest = {
    ids: number[];
  };

  type BatchSendEmailTask = {
    id: number;
    subject: string;
    content: string;
    recipients: string;
    scope: number;
    register_start_time: number;
    register_end_time: number;
    additional: string;
    scheduled: number;
    interval: number;
    limit: number;
    status: number;
    errors: string;
    total: number;
    current: number;
    created_at: number;
    updated_at: number;
  };

  type CheckoutOrderRequest = {
    orderNo: string;
    returnUrl?: string;
  };

  type CheckoutOrderResponse = {
    type: string;
    checkout_url?: string;
    stripe?: StripePayment;
  };

  type CloseOrderRequest = {
    orderNo: string;
  };

  type CommissionLog = {
    type: number;
    user_id: number;
    amount: number;
    order_no: string;
    timestamp: number;
  };

  type Coupon = {
    id: number;
    name: string;
    code: string;
    count: number;
    type: number;
    discount: number;
    start_time: number;
    expire_time: number;
    user_limit: number;
    subscribe: number[];
    used_count: number;
    enable: boolean;
    created_at: number;
    updated_at: number;
  };

  type CreateAdsRequest = {
    title: string;
    type: string;
    content: string;
    description: string;
    target_url: string;
    start_time: number;
    end_time: number;
    status: number;
  };

  type CreateAnnouncementRequest = {
    title: string;
    content: string;
  };

  type CreateBatchSendEmailTaskRequest = {
    subject: string;
    content: string;
    scope: number;
    register_start_time?: number;
    register_end_time?: number;
    additional?: string;
    scheduled?: number;
    interval?: number;
    limit?: number;
  };

  type CreateCouponRequest = {
    name: string;
    code?: string;
    count?: number;
    type: number;
    discount: number;
    start_time: number;
    expire_time: number;
    user_limit?: number;
    subscribe?: number[];
    used_count?: number;
    enable?: boolean;
  };

  type CreateDocumentRequest = {
    title: string;
    content: string;
    tags?: string[];
    show: boolean;
  };

  type CreateNodeRequest = {
    name: string;
    tags?: string[];
    port: number;
    address: string;
    server_id: number;
    protocol: string;
    enabled: boolean;
  };

  type CreateOrderRequest = {
    user_id: number;
    type: number;
    quantity?: number;
    price: number;
    amount: number;
    discount?: number;
    coupon?: string;
    coupon_discount?: number;
    commission: number;
    fee_amount: number;
    payment_id: number;
    trade_no?: string;
    status?: number;
    subscribe_id?: number;
  };

  type CreatePaymentMethodRequest = {
    name: string;
    platform: string;
    description: string;
    icon?: string;
    domain?: string;
    config: Record<string, any>;
    fee_mode: number;
    fee_percent?: number;
    fee_amount?: number;
    enable: boolean;
  };

  type CreateQuotaTaskRequest = {
    subscribers: number[];
    is_active: boolean;
    start_time: number;
    end_time: number;
    reset_traffic: boolean;
    days: number;
    gift_type: number;
    gift_value: number;
  };

  type CreateServerRequest = {
    name: string;
    country?: string;
    city?: string;
    address: string;
    sort?: number;
    protocols: Protocol[];
  };

  type CreateSubscribeApplicationRequest = {
    name: string;
    description?: string;
    icon?: string;
    scheme?: string;
    user_agent: string;
    is_default: boolean;
    template: string;
    output_format: string;
    download_link: DownloadLink;
  };

  type CreateSubscribeGroupRequest = {
    name: string;
    description: string;
  };

  type CreateSubscribeRequest = {
    name: string;
    language: string;
    description: string;
    unit_price: number;
    unit_time: string;
    discount: SubscribeDiscount[];
    replacement: number;
    inventory: number;
    traffic: number;
    speed_limit: number;
    device_limit: number;
    quota: number;
    nodes: number[];
    node_tags: string[];
    show: boolean;
    sell: boolean;
    deduction_ratio: number;
    allow_deduction: boolean;
    reset_cycle: number;
    renewal_reset: boolean;
  };

  type CreateTicketFollowRequest = {
    ticket_id: number;
    from: string;
    type: number;
    content: string;
  };

  type CreateUserAuthMethodRequest = {
    user_id: number;
    auth_type: string;
    auth_identifier: string;
  };

  type CreateUserRequest = {
    email: string;
    telephone: string;
    telephone_area_code: string;
    password: string;
    product_id: number;
    duration: number;
    referral_percentage: number;
    only_first_purchase: boolean;
    referer_user: string;
    refer_code: string;
    balance: number;
    commission: number;
    gift_amount: number;
    is_admin: boolean;
  };

  type CreateUserSubscribeRequest = {
    user_id: number;
    expired_at: number;
    traffic: number;
    subscribe_id: number;
  };

  type CurrencyConfig = {
    access_key: string;
    currency_unit: string;
    currency_symbol: string;
  };

  type DeleteAdsRequest = {
    id: number;
  };

  type DeleteAnnouncementRequest = {
    id: number;
  };

  type DeleteCouponRequest = {
    id: number;
  };

  type DeleteDocumentRequest = {
    id: number;
  };

  type DeleteNodeRequest = {
    id: number;
  };

  type DeletePaymentMethodRequest = {
    id: number;
  };

  type DeleteServerRequest = {
    id: number;
  };

  type DeleteSubscribeApplicationRequest = {
    id: number;
  };

  type DeleteSubscribeGroupRequest = {
    id: number;
  };

  type DeleteSubscribeRequest = {
    id: number;
  };

  type DeleteUserAuthMethodRequest = {
    user_id: number;
    auth_type: string;
  };

  type DeleteUserDeivceRequest = {
    id: number;
  };

  type DeleteUserParams = {
    id: number;
  };

  type DeleteUserSubscribeRequest = {
    user_subscribe_id: number;
  };

  type DeviceAuthticateConfig = {
    enable: boolean;
    show_ads: boolean;
    enable_security: boolean;
    only_real_device: boolean;
  };

  type Document = {
    id: number;
    title: string;
    content: string;
    tags: string[];
    show: boolean;
    created_at: number;
    updated_at: number;
  };

  type DownloadLink = {
    ios?: string;
    android?: string;
    windows?: string;
    mac?: string;
    linux?: string;
    harmony?: string;
  };

  type EmailAuthticateConfig = {
    enable: boolean;
    enable_verify: boolean;
    enable_domain_suffix: boolean;
    domain_suffix_list: string;
  };

  type EPayNotifyRequest = {
    pid: number;
    trade_no: string;
    out_trade_no: string;
    type: string;
    name: string;
    money: string;
    trade_status: string;
    param: string;
    sign: string;
    sign_type: string;
  };

  type FilterBalanceLogParams = {
    page: number;
    size: number;
    date?: string;
    search?: string;
    user_id?: number;
  };

  type FilterBalanceLogRequest = {
    page?: number;
    size?: number;
    date?: string;
    search?: string;
    user_id?: number;
  };

  type FilterBalanceLogResponse = {
    total: number;
    list: BalanceLog[];
  };

  type FilterCommissionLogParams = {
    page: number;
    size: number;
    date?: string;
    search?: string;
    user_id?: number;
  };

  type FilterCommissionLogRequest = {
    page?: number;
    size?: number;
    date?: string;
    search?: string;
    user_id?: number;
  };

  type FilterCommissionLogResponse = {
    total: number;
    list: CommissionLog[];
  };

  type FilterEmailLogParams = {
    page: number;
    size: number;
    date?: string;
    search?: string;
  };

  type FilterEmailLogResponse = {
    total: number;
    list: MessageLog[];
  };

  type FilterGiftLogParams = {
    page: number;
    size: number;
    date?: string;
    search?: string;
    user_id?: number;
  };

  type FilterGiftLogRequest = {
    page?: number;
    size?: number;
    date?: string;
    search?: string;
    user_id?: number;
  };

  type FilterGiftLogResponse = {
    total: number;
    list: GiftLog[];
  };

  type FilterLoginLogParams = {
    page: number;
    size: number;
    date?: string;
    search?: string;
    user_id?: number;
  };

  type FilterLoginLogRequest = {
    page?: number;
    size?: number;
    date?: string;
    search?: string;
    user_id?: number;
  };

  type FilterLoginLogResponse = {
    total: number;
    list: LoginLog[];
  };

  type FilterLogParams = {
    page: number;
    size: number;
    date?: string;
    search?: string;
  };

  type FilterMobileLogParams = {
    page: number;
    size: number;
    date?: string;
    search?: string;
  };

  type FilterMobileLogResponse = {
    total: number;
    list: MessageLog[];
  };

  type FilterNodeListParams = {
    page: number;
    size: number;
    search?: string;
  };

  type FilterNodeListRequest = {
    page: number;
    size: number;
    search?: string;
  };

  type FilterNodeListResponse = {
    total: number;
    list: Node[];
  };

  type FilterRegisterLogParams = {
    page: number;
    size: number;
    date?: string;
    search?: string;
    user_id?: number;
  };

  type FilterRegisterLogRequest = {
    page?: number;
    size?: number;
    date?: string;
    search?: string;
    user_id?: number;
  };

  type FilterRegisterLogResponse = {
    total: number;
    list: RegisterLog[];
  };

  type FilterResetSubscribeLogParams = {
    page: number;
    size: number;
    date?: string;
    search?: string;
    user_subscribe_id?: number;
  };

  type FilterResetSubscribeLogRequest = {
    page?: number;
    size?: number;
    date?: string;
    search?: string;
    user_subscribe_id?: number;
  };

  type FilterResetSubscribeLogResponse = {
    total: number;
    list: ResetSubscribeLog[];
  };

  type FilterServerListParams = {
    page: number;
    size: number;
    search?: string;
  };

  type FilterServerListRequest = {
    page: number;
    size: number;
    search?: string;
  };

  type FilterServerListResponse = {
    total: number;
    list: Server[];
  };

  type FilterServerTrafficLogParams = {
    page: number;
    size: number;
    date?: string;
    search?: string;
    server_id?: number;
  };

  type FilterServerTrafficLogRequest = {
    page?: number;
    size?: number;
    date?: string;
    search?: string;
    server_id?: number;
  };

  type FilterServerTrafficLogResponse = {
    total: number;
    list: ServerTrafficLog[];
  };

  type FilterSubscribeLogParams = {
    page: number;
    size: number;
    date?: string;
    search?: string;
    user_id?: number;
    user_subscribe_id?: number;
  };

  type FilterSubscribeLogRequest = {
    page?: number;
    size?: number;
    date?: string;
    search?: string;
    user_id?: number;
    user_subscribe_id?: number;
  };

  type FilterSubscribeLogResponse = {
    total: number;
    list: SubscribeLog[];
  };

  type FilterSubscribeTrafficRequest = {
    page?: number;
    size?: number;
    date?: string;
    search?: string;
    user_id?: number;
    user_subscribe_id?: number;
  };

  type FilterSubscribeTrafficResponse = {
    total: number;
    list: UserSubscribeTrafficLog[];
  };

  type FilterTrafficLogDetailsParams = {
    page: number;
    size: number;
    date?: string;
    search?: string;
    server_id?: number;
    subscribe_id?: number;
    user_id?: number;
  };

  type FilterTrafficLogDetailsRequest = {
    page?: number;
    size?: number;
    date?: string;
    search?: string;
    server_id?: number;
    subscribe_id?: number;
    user_id?: number;
  };

  type FilterTrafficLogDetailsResponse = {
    total: number;
    list: TrafficLogDetails[];
  };

  type FilterUserSubscribeTrafficLogParams = {
    page: number;
    size: number;
    date?: string;
    search?: string;
    user_id?: number;
    user_subscribe_id?: number;
  };

  type Follow = {
    id: number;
    ticket_id: number;
    from: string;
    type: number;
    content: string;
    created_at: number;
  };

  type GetAdsDetailParams = {
    id: number;
  };

  type GetAdsDetailRequest = {
    id: number;
  };

  type GetAdsListParams = {
    page: number;
    size: number;
    status?: number;
    search?: string;
  };

  type GetAdsListRequest = {
    page: number;
    size: number;
    status?: number;
    search?: string;
  };

  type GetAdsListResponse = {
    total: number;
    list: Ads[];
  };

  type GetAnnouncementListParams = {
    page: number;
    size: number;
    show?: boolean;
    pinned?: boolean;
    popup?: boolean;
    search?: string;
  };

  type GetAnnouncementListRequest = {
    page: number;
    size: number;
    show?: boolean;
    pinned?: boolean;
    popup?: boolean;
    search?: string;
  };

  type GetAnnouncementListResponse = {
    total: number;
    list: Announcement[];
  };

  type GetAnnouncementParams = {
    id: number;
  };

  type GetAnnouncementRequest = {
    id: number;
  };

  type GetAuthMethodConfigParams = {
    method: string;
  };

  type GetAuthMethodConfigRequest = {
    method: string;
  };

  type GetAuthMethodListResponse = {
    list: AuthMethodConfig[];
  };

  type GetAvailablePaymentMethodsResponse = {
    list: PaymentMethod[];
  };

  type GetBatchSendEmailTaskListParams = {
    page: number;
    size: number;
    scope?: number;
    status?: number;
  };

  type GetBatchSendEmailTaskListRequest = {
    page: number;
    size: number;
    scope?: number;
    status?: number;
  };

  type GetBatchSendEmailTaskListResponse = {
    total: number;
    list: BatchSendEmailTask[];
  };

  type GetBatchSendEmailTaskStatusRequest = {
    id: number;
  };

  type GetBatchSendEmailTaskStatusResponse = {
    status: number;
    current: number;
    total: number;
    errors: string;
  };

  type GetCouponListParams = {
    page: number;
    size: number;
    subscribe?: number;
    search?: string;
  };

  type GetCouponListRequest = {
    page: number;
    size: number;
    subscribe?: number;
    search?: string;
  };

  type GetCouponListResponse = {
    total: number;
    list: Coupon[];
  };

  type GetDetailRequest = {
    id: number;
  };

  type GetDocumentDetailRequest = {
    id: number;
  };

  type GetDocumentListParams = {
    page: number;
    size: number;
    tag?: string;
    search?: string;
  };

  type GetDocumentListRequest = {
    page: number;
    size: number;
    tag?: string;
    search?: string;
  };

  type GetDocumentListResponse = {
    total: number;
    list: Document[];
  };

  type GetMessageLogListParams = {
    page: number;
    size: number;
    type: number;
    search?: string;
  };

  type GetMessageLogListRequest = {
    page: number;
    size: number;
    type: number;
    search?: string;
  };

  type GetMessageLogListResponse = {
    total: number;
    list: MessageLog[];
  };

  type GetNodeMultiplierResponse = {
    periods: TimePeriod[];
  };

  type GetOrderListParams = {
    page: number;
    size: number;
    user_id?: number;
    status?: number;
    subscribe_id?: number;
    search?: string;
  };

  type GetOrderListRequest = {
    page: number;
    size: number;
    user_id?: number;
    status?: number;
    subscribe_id?: number;
    search?: string;
  };

  type GetOrderListResponse = {
    total: number;
    list: Order[];
  };

  type GetPaymentMethodListParams = {
    page: number;
    size: number;
    platform?: string;
    search?: string;
    enable?: boolean;
  };

  type GetPaymentMethodListRequest = {
    page: number;
    size: number;
    platform?: string;
    search?: string;
    enable?: boolean;
  };

  type GetPaymentMethodListResponse = {
    total: number;
    list: PaymentMethodDetail[];
  };

  type GetPreSendEmailCountRequest = {
    scope: number;
    register_start_time?: number;
    register_end_time?: number;
  };

  type GetPreSendEmailCountResponse = {
    count: number;
  };

  type GetServerProtocolsParams = {
    id: number;
  };

  type GetServerProtocolsRequest = {
    id: number;
  };

  type GetServerProtocolsResponse = {
    protocols: Protocol[];
  };

  type GetSubscribeApplicationListParams = {
    page: number;
    size: number;
  };

  type GetSubscribeApplicationListRequest = {
    page: number;
    size: number;
  };

  type GetSubscribeApplicationListResponse = {
    total: number;
    list: SubscribeApplication[];
  };

  type GetSubscribeDetailsParams = {
    id: number;
  };

  type GetSubscribeDetailsRequest = {
    id: number;
  };

  type GetSubscribeGroupListResponse = {
    list: SubscribeGroup[];
    total: number;
  };

  type GetSubscribeListParams = {
    page: number;
    size: number;
    language?: string;
    search?: string;
  };

  type GetSubscribeListRequest = {
    page: number;
    size: number;
    language?: string;
    search?: string;
  };

  type GetSubscribeListResponse = {
    list: SubscribeItem[];
    total: number;
  };

  type GetTicketListParams = {
    page: number;
    size: number;
    user_id?: number;
    status?: number;
    search?: string;
  };

  type GetTicketListRequest = {
    page: number;
    size: number;
    user_id?: number;
    status?: number;
    search?: string;
  };

  type GetTicketListResponse = {
    total: number;
    list: Ticket[];
  };

  type GetTicketParams = {
    id: number;
  };

  type GetTicketRequest = {
    id: number;
  };

  type GetUserAuthMethodRequest = {
    user_id: number;
  };

  type GetUserAuthMethodResponse = {
    auth_methods: UserAuthMethod[];
  };

  type GetUserDetailParams = {
    id: number;
  };

  type GetUserListParams = {
    page: number;
    size: number;
    search?: string;
    user_id?: number;
    subscribe_id?: number;
    user_subscribe_id?: number;
  };

  type GetUserListRequest = {
    page: number;
    size: number;
    search?: string;
    user_id?: number;
    subscribe_id?: number;
    user_subscribe_id?: number;
  };

  type GetUserListResponse = {
    total: number;
    list: User[];
  };

  type GetUserLoginLogsParams = {
    page: number;
    size: number;
    user_id: number;
  };

  type GetUserLoginLogsRequest = {
    page: number;
    size: number;
    user_id: number;
  };

  type GetUserLoginLogsResponse = {
    list: UserLoginLog[];
    total: number;
  };

  type GetUserSubscribeByIdParams = {
    id: number;
  };

  type GetUserSubscribeByIdRequest = {
    id: number;
  };

  type GetUserSubscribeDevicesParams = {
    page: number;
    size: number;
    user_id: number;
    subscribe_id: number;
  };

  type GetUserSubscribeDevicesRequest = {
    page: number;
    size: number;
    user_id: number;
    subscribe_id: number;
  };

  type GetUserSubscribeDevicesResponse = {
    list: UserDevice[];
    total: number;
  };

  type GetUserSubscribeListRequest = {
    page: number;
    size: number;
    user_id: number;
  };

  type GetUserSubscribeListResponse = {
    list: UserSubscribe[];
    total: number;
  };

  type GetUserSubscribeLogsParams = {
    page: number;
    size: number;
    user_id: number;
    subscribe_id?: number;
  };

  type GetUserSubscribeLogsRequest = {
    page: number;
    size: number;
    user_id: number;
    subscribe_id?: number;
  };

  type GetUserSubscribeLogsResponse = {
    list: UserSubscribeLog[];
    total: number;
  };

  type GetUserSubscribeParams = {
    page: number;
    size: number;
    user_id: number;
  };

  type GetUserSubscribeResetTrafficLogsParams = {
    page: number;
    size: number;
    user_subscribe_id: number;
  };

  type GetUserSubscribeResetTrafficLogsRequest = {
    page: number;
    size: number;
    user_subscribe_id: number;
  };

  type GetUserSubscribeResetTrafficLogsResponse = {
    list: ResetSubscribeTrafficLog[];
    total: number;
  };

  type GetUserSubscribeTrafficLogsParams = {
    page: number;
    size: number;
    user_id: number;
    subscribe_id: number;
    start_time: number;
    end_time: number;
  };

  type GetUserSubscribeTrafficLogsRequest = {
    page: number;
    size: number;
    user_id: number;
    subscribe_id: number;
    start_time: number;
    end_time: number;
  };

  type GetUserSubscribeTrafficLogsResponse = {
    list: TrafficLog[];
    total: number;
  };

  type GiftLog = {
    type: number;
    user_id: number;
    order_no: string;
    subscribe_id: number;
    amount: number;
    balance: number;
    remark?: string;
    timestamp: number;
  };

  type HasMigrateSeverNodeResponse = {
    has_migrate: boolean;
  };

  type Hysteria2 = {
    port: number;
    hop_ports: string;
    hop_interval: number;
    obfs_password: string;
    security_config: SecurityConfig;
  };

  type InviteConfig = {
    forced_invite: boolean;
    referral_percentage: number;
    only_first_purchase: boolean;
  };

  type KickOfflineRequest = {
    id: number;
  };

  type LoginLog = {
    user_id: number;
    method: string;
    login_ip: string;
    user_agent: string;
    success: boolean;
    timestamp: number;
  };

  type LogResponse = {
    list: Record<string, any>;
  };

  type LogSetting = {
    auto_clear: boolean;
    clear_days: number;
  };

  type MessageLog = {
    id: number;
    type: number;
    platform: string;
    to: string;
    subject: string;
    content: Record<string, any>;
    status: number;
    created_at: number;
  };

  type MigrateServerNodeResponse = {
    succee: number;
    fail: number;
    message?: string;
  };

  type MobileAuthenticateConfig = {
    enable: boolean;
    enable_whitelist: boolean;
    whitelist: string[];
  };

  type Node = {
    id: number;
    name: string;
    tags: string[];
    port: number;
    address: string;
    server_id: number;
    protocol: string;
    enabled: boolean;
    sort?: number;
    created_at: number;
    updated_at: number;
  };

  type NodeConfig = {
    node_secret: string;
    node_pull_interval: number;
    node_push_interval: number;
    traffic_report_threshold: number;
    ip_strategy: string;
    dns: NodeDNS[];
    block: string[];
    outbound: NodeOutbound[];
  };

  type NodeDNS = {
    proto: string;
    address: string;
    domains: string[];
  };

  type NodeOutbound = {
    name: string;
    protocol: string;
    address: string;
    port: number;
    password: string;
    rules: string[];
  };

  type NodeRelay = {
    host: string;
    port: number;
    prefix: string;
  };

  type Order = {
    id: number;
    user_id: number;
    order_no: string;
    type: number;
    quantity: number;
    price: number;
    amount: number;
    gift_amount: number;
    discount: number;
    coupon: string;
    coupon_discount: number;
    commission?: number;
    payment: PaymentMethod;
    fee_amount: number;
    trade_no: string;
    status: number;
    subscribe_id: number;
    created_at: number;
    updated_at: number;
  };

  type OrderDetail = {
    id: number;
    user_id: number;
    order_no: string;
    type: number;
    quantity: number;
    price: number;
    amount: number;
    gift_amount: number;
    discount: number;
    coupon: string;
    coupon_discount: number;
    commission?: number;
    payment: PaymentMethod;
    method: string;
    fee_amount: number;
    trade_no: string;
    status: number;
    subscribe_id: number;
    subscribe: Subscribe;
    created_at: number;
    updated_at: number;
  };

  type OrdersStatistics = {
    date?: string;
    amount_total: number;
    new_order_amount: number;
    renewal_order_amount: number;
    list?: OrdersStatistics[];
  };

  type PaymentConfig = {
    id: number;
    name: string;
    platform: string;
    description: string;
    icon?: string;
    domain?: string;
    config: Record<string, any>;
    fee_mode: number;
    fee_percent?: number;
    fee_amount?: number;
    enable: boolean;
  };

  type PaymentMethod = {
    id: number;
    name: string;
    platform: string;
    description: string;
    icon: string;
    fee_mode: number;
    fee_percent: number;
    fee_amount: number;
  };

  type PaymentMethodDetail = {
    id: number;
    name: string;
    platform: string;
    description: string;
    icon: string;
    domain: string;
    config: Record<string, any>;
    fee_mode: number;
    fee_percent: number;
    fee_amount: number;
    enable: boolean;
    notify_url: string;
  };

  type PlatformInfo = {
    platform: string;
    platform_url: string;
    platform_field_description: Record<string, any>;
  };

  type PlatformResponse = {
    list: PlatformInfo[];
  };

  type PreOrderResponse = {
    price: number;
    amount: number;
    discount: number;
    gift_amount: number;
    coupon: string;
    coupon_discount: number;
    fee_amount: number;
  };

  type PreRenewalOrderResponse = {
    orderNo: string;
  };

  type PreViewNodeMultiplierResponse = {
    current_time: string;
    ratio: number;
  };

  type PreviewSubscribeTemplateParams = {
    id: number;
  };

  type PreviewSubscribeTemplateRequest = {
    id: number;
  };

  type PreviewSubscribeTemplateResponse = {
    /** 预览的模板内容 */
    template: string;
  };

  type PrivacyPolicyConfig = {
    privacy_policy: string;
  };

  type Protocol = {
    type: string;
    port: number;
    enable: boolean;
    security?: string;
    sni?: string;
    allow_insecure?: boolean;
    fingerprint?: string;
    reality_server_addr?: string;
    reality_server_port?: number;
    reality_private_key?: string;
    reality_public_key?: string;
    reality_short_id?: string;
    transport?: string;
    host?: string;
    path?: string;
    service_name?: string;
    cipher?: string;
    server_key?: string;
    flow?: string;
    hop_ports?: string;
    hop_interval?: number;
    obfs_password?: string;
    disable_sni?: boolean;
    reduce_rtt?: boolean;
    udp_relay_mode?: string;
    congestion_controller?: string;
    /** mux, eg: off/low/medium/high */
    multiplex?: string;
    /** padding scheme */
    padding_scheme?: string;
    /** upload speed limit */
    up_mbps?: number;
    /** download speed limit */
    down_mbps?: number;
    /** obfs, 'none', 'http', 'tls' */
    obfs?: string;
    /** obfs host */
    obfs_host?: string;
    /** obfs path */
    obfs_path?: string;
    /** xhttp mode */
    xhttp_mode?: string;
    /** xhttp extra path */
    xhttp_extra?: string;
    /** encryption，'none', 'mlkem768x25519plus' */
    encryption?: string;
    /** encryption mode，'native', 'xorpub', 'random' */
    encryption_mode?: string;
    /** encryption rtt，'0rtt', '1rtt' */
    encryption_rtt?: string;
    /** encryption ticket */
    encryption_ticket?: string;
    /** encryption server padding */
    encryption_server_padding?: string;
    /** encryption private key */
    encryption_private_key?: string;
    /** encryption client padding */
    encryption_client_padding?: string;
    /** encryption password */
    encryption_password?: string;
    /** Traffic ratio, default is 1 */
    ratio?: number;
    /** Certificate mode, `none`｜`http`｜`dns`｜`self` */
    cert_mode?: string;
    /** DNS provider for certificate */
    cert_dns_provider?: string;
    /** Environment for DNS provider */
    cert_dns_env?: string;
  };

  type PubilcRegisterConfig = {
    stop_register: boolean;
    enable_ip_register_limit: boolean;
    ip_register_limit: number;
    ip_register_limit_duration: number;
  };

  type PubilcVerifyCodeConfig = {
    verify_code_interval: number;
  };

  type PurchaseOrderRequest = {
    subscribe_id: number;
    quantity: number;
    payment?: number;
    coupon?: string;
  };

  type PurchaseOrderResponse = {
    order_no: string;
  };

  type QueryAnnouncementRequest = {
    page: number;
    size: number;
    pinned: boolean;
    popup: boolean;
  };

  type QueryAnnouncementResponse = {
    total: number;
    announcements: Announcement[];
  };

  type QueryDocumentDetailRequest = {
    id: number;
  };

  type QueryDocumentListResponse = {
    total: number;
    list: Document[];
  };

  type QueryNodeTagResponse = {
    tags: string[];
  };

  type QueryOrderDetailRequest = {
    order_no: string;
  };

  type QueryOrderListRequest = {
    page: number;
    size: number;
  };

  type QueryOrderListResponse = {
    total: number;
    list: OrderDetail[];
  };

  type QueryQuotaTaskListParams = {
    page: number;
    size: number;
    status?: number;
  };

  type QueryQuotaTaskListRequest = {
    page: number;
    size: number;
    status?: number;
  };

  type QueryQuotaTaskListResponse = {
    total: number;
    list: QuotaTask[];
  };

  type QueryQuotaTaskPreCountRequest = {
    subscribers: number[];
    is_active: boolean;
    start_time: number;
    end_time: number;
  };

  type QueryQuotaTaskPreCountResponse = {
    count: number;
  };

  type QueryQuotaTaskStatusRequest = {
    id: number;
  };

  type QueryQuotaTaskStatusResponse = {
    status: number;
    current: number;
    total: number;
    errors: string;
  };

  type QuerySubscribeGroupListResponse = {
    list: SubscribeGroup[];
    total: number;
  };

  type QuerySubscribeListResponse = {
    list: Subscribe[];
    total: number;
  };

  type QueryUserAffiliateCountResponse = {
    registers: number;
    total_commission: number;
  };

  type QueryUserAffiliateListRequest = {
    page: number;
    size: number;
  };

  type QueryUserAffiliateListResponse = {
    list: UserAffiliate[];
    total: number;
  };

  type QuotaTask = {
    id: number;
    subscribers: number[];
    is_active: boolean;
    start_time: number;
    end_time: number;
    reset_traffic: boolean;
    days: number;
    gift_type: number;
    gift_value: number;
    /** UserSubscribe IDs */
    objects: number[];
    status: number;
    total: number;
    current: number;
    errors: string;
    created_at: number;
    updated_at: number;
  };

  type RechargeOrderRequest = {
    amount: number;
    payment: number;
  };

  type RechargeOrderResponse = {
    order_no: string;
  };

  type RegisterConfig = {
    stop_register: boolean;
    enable_trial: boolean;
    trial_subscribe: number;
    trial_time: number;
    trial_time_unit: string;
    enable_ip_register_limit: boolean;
    ip_register_limit: number;
    ip_register_limit_duration: number;
  };

  type RegisterLog = {
    user_id: number;
    auth_method: string;
    identifier: string;
    register_ip: string;
    user_agent: string;
    timestamp: number;
  };

  type RenewalOrderRequest = {
    user_subscribe_id: number;
    quantity: number;
    payment: number;
    coupon?: string;
  };

  type RenewalOrderResponse = {
    order_no: string;
  };

  type ResetSortRequest = {
    sort: SortItem[];
  };

  type ResetSubscribeLog = {
    type: number;
    user_id: number;
    user_subscribe_id: number;
    order_no?: string;
    timestamp: number;
  };

  type ResetSubscribeTrafficLog = {
    id: number;
    type: number;
    user_subscribe_id: number;
    order_no?: string;
    timestamp: number;
  };

  type ResetTrafficOrderRequest = {
    user_subscribe_id: number;
    payment: number;
  };

  type ResetTrafficOrderResponse = {
    order_no: string;
  };

  type Response = {
    /** 状态码 */
    code?: number;
    /** 消息 */
    msg?: string;
    /** 数据 */
    data?: Record<string, any>;
  };

  type RevenueStatisticsResponse = {
    today: OrdersStatistics;
    monthly: OrdersStatistics;
    all: OrdersStatistics;
  };

  type SecurityConfig = {
    sni: string;
    allow_insecure: boolean;
    fingerprint: string;
    reality_server_addr: string;
    reality_server_port: number;
    reality_private_key: string;
    reality_public_key: string;
    reality_short_id: string;
  };

  type Server = {
    id: number;
    name: string;
    country: string;
    city: string;
    address: string;
    sort: number;
    protocols: Protocol[];
    last_reported_at: number;
    status: ServerStatus;
    created_at: number;
    updated_at: number;
  };

  type ServerGroup = {
    id: number;
    name: string;
    description: string;
    created_at: number;
    updated_at: number;
  };

  type ServerOnlineIP = {
    ip: string;
    protocol: string;
  };

  type ServerOnlineUser = {
    ip: ServerOnlineIP[];
    user_id: number;
    subscribe: string;
    subscribe_id: number;
    traffic: number;
    expired_at: number;
  };

  type ServerRuleGroup = {
    id: number;
    icon: string;
    name: string;
    type: string;
    tags: string[];
    rules: string;
    enable: boolean;
    default: boolean;
    created_at: number;
    updated_at: number;
  };

  type ServerStatus = {
    cpu: number;
    mem: number;
    disk: number;
    protocol: string;
    online: ServerOnlineUser[];
    status: string;
  };

  type ServerTotalDataResponse = {
    online_users: number;
    online_servers: number;
    offline_servers: number;
    today_upload: number;
    today_download: number;
    monthly_upload: number;
    monthly_download: number;
    updated_at: number;
    server_traffic_ranking_today: ServerTrafficData[];
    server_traffic_ranking_yesterday: ServerTrafficData[];
    user_traffic_ranking_today: UserTrafficData[];
    user_traffic_ranking_yesterday: UserTrafficData[];
  };

  type ServerTrafficData = {
    server_id: number;
    name: string;
    upload: number;
    download: number;
  };

  type ServerTrafficLog = {
    /** Server ID */
    server_id: number;
    /** Upload traffic in bytes */
    upload: number;
    /** Download traffic in bytes */
    download: number;
    /** Total traffic in bytes (Upload + Download) */
    total: number;
    /** Date in YYYY-MM-DD format */
    date: string;
    /** Whether to show detailed traffic */
    details: boolean;
  };

  type SetNodeMultiplierRequest = {
    periods: TimePeriod[];
  };

  type Shadowsocks = {
    method: string;
    port: number;
    server_key: string;
  };

  type SiteConfig = {
    host: string;
    site_name: string;
    site_desc: string;
    site_logo: string;
    keywords: string;
    custom_html: string;
    custom_data: string;
  };

  type SiteCustomDataContacts = {
    email: string;
    telephone: string;
    address: string;
  };

  type SortItem = {
    id: number;
    sort: number;
  };

  type StopBatchSendEmailTaskRequest = {
    id: number;
  };

  type StripePayment = {
    method: string;
    client_secret: string;
    publishable_key: string;
  };

  type Subscribe = {
    id: number;
    name: string;
    language: string;
    description: string;
    unit_price: number;
    unit_time: string;
    discount: SubscribeDiscount[];
    replacement: number;
    inventory: number;
    traffic: number;
    speed_limit: number;
    device_limit: number;
    quota: number;
    nodes: number[];
    node_tags: string[];
    show: boolean;
    sell: boolean;
    sort: number;
    deduction_ratio: number;
    allow_deduction: boolean;
    reset_cycle: number;
    renewal_reset: boolean;
    created_at: number;
    updated_at: number;
  };

  type SubscribeApplication = {
    id: number;
    name: string;
    description?: string;
    icon?: string;
    scheme?: string;
    user_agent: string;
    is_default: boolean;
    template: string;
    output_format: string;
    download_link?: DownloadLink;
    created_at: number;
    updated_at: number;
  };

  type SubscribeConfig = {
    single_model: boolean;
    subscribe_path: string;
    subscribe_domain: string;
    pan_domain: boolean;
    user_agent_limit: boolean;
    user_agent_list: string;
  };

  type SubscribeDiscount = {
    quantity: number;
    discount: number;
  };

  type SubscribeGroup = {
    id: number;
    name: string;
    description: string;
    created_at: number;
    updated_at: number;
  };

  type SubscribeItem = {
    id?: number;
    name?: string;
    language?: string;
    description?: string;
    unit_price?: number;
    unit_time?: string;
    discount?: SubscribeDiscount[];
    replacement?: number;
    inventory?: number;
    traffic?: number;
    speed_limit?: number;
    device_limit?: number;
    quota?: number;
    nodes?: number[];
    node_tags?: string[];
    show?: boolean;
    sell?: boolean;
    sort?: number;
    deduction_ratio?: number;
    allow_deduction?: boolean;
    reset_cycle?: number;
    renewal_reset?: boolean;
    created_at?: number;
    updated_at?: number;
    sold: number;
  };

  type SubscribeLog = {
    user_id: number;
    token: string;
    user_agent: string;
    client_ip: string;
    user_subscribe_id: number;
    timestamp: number;
  };

  type SubscribeSortRequest = {
    sort: SortItem[];
  };

  type SubscribeType = {
    subscribe_types: string[];
  };

  type TelegramConfig = {
    telegram_bot_token: string;
    telegram_group_url: string;
    telegram_notify: boolean;
    telegram_web_hook_domain: string;
  };

  type TestEmailSendRequest = {
    email: string;
  };

  type TestSmsSendRequest = {
    area_code: string;
    telephone: string;
  };

  type Ticket = {
    id: number;
    title: string;
    description: string;
    user_id: number;
    follow?: Follow[];
    status: number;
    created_at: number;
    updated_at: number;
  };

  type TicketWaitRelpyResponse = {
    count: number;
  };

  type TimePeriod = {
    start_time: string;
    end_time: string;
    multiplier: number;
  };

  type ToggleNodeStatusRequest = {
    id: number;
    enable: boolean;
  };

  type TosConfig = {
    tos_content: string;
  };

  type TrafficLog = {
    id: number;
    server_id: number;
    user_id: number;
    subscribe_id: number;
    download: number;
    upload: number;
    timestamp: number;
  };

  type TrafficLogDetails = {
    id: number;
    server_id: number;
    user_id: number;
    subscribe_id: number;
    download: number;
    upload: number;
    timestamp: number;
  };

  type TransportConfig = {
    path: string;
    host: string;
    service_name: string;
  };

  type Trojan = {
    port: number;
    transport: string;
    transport_config: TransportConfig;
    security: string;
    security_config: SecurityConfig;
  };

  type Tuic = {
    port: number;
    disable_sni: boolean;
    reduce_rtt: boolean;
    udp_relay_mode: string;
    congestion_controller: string;
    security_config: SecurityConfig;
  };

  type UpdateAdsRequest = {
    id: number;
    title: string;
    type: string;
    content: string;
    description: string;
    target_url: string;
    start_time: number;
    end_time: number;
    status: number;
  };

  type UpdateAnnouncementEnableRequest = {
    id: number;
    enable: boolean;
  };

  type UpdateAnnouncementRequest = {
    id: number;
    title: string;
    content: string;
    show: boolean;
    pinned: boolean;
    popup: boolean;
  };

  type UpdateAuthMethodConfigRequest = {
    id: number;
    method: string;
    config: Record<string, any>;
    enabled: boolean;
  };

  type UpdateCouponRequest = {
    id: number;
    name: string;
    code?: string;
    count?: number;
    type: number;
    discount: number;
    start_time: number;
    expire_time: number;
    user_limit?: number;
    subscribe?: number[];
    used_count?: number;
    enable?: boolean;
  };

  type UpdateDocumentRequest = {
    id: number;
    title: string;
    content: string;
    tags?: string[];
    show: boolean;
  };

  type UpdateNodeRequest = {
    id: number;
    name: string;
    tags?: string[];
    port: number;
    address: string;
    server_id: number;
    protocol: string;
    enabled: boolean;
  };

  type UpdateOrderStatusRequest = {
    id: number;
    status: number;
    payment_id?: number;
    trade_no?: string;
  };

  type UpdatePaymentMethodRequest = {
    id: number;
    name: string;
    platform: string;
    description: string;
    icon?: string;
    domain?: string;
    config: Record<string, any>;
    fee_mode: number;
    fee_percent?: number;
    fee_amount?: number;
    enable: boolean;
  };

  type UpdateServerRequest = {
    id: number;
    name: string;
    country?: string;
    city?: string;
    address: string;
    sort?: number;
    protocols: Protocol[];
  };

  type UpdateSubscribeApplicationRequest = {
    id: number;
    name: string;
    description?: string;
    icon?: string;
    scheme?: string;
    user_agent: string;
    is_default: boolean;
    template: string;
    output_format: string;
    download_link?: DownloadLink;
  };

  type UpdateSubscribeGroupRequest = {
    id: number;
    name: string;
    description: string;
  };

  type UpdateSubscribeRequest = {
    id: number;
    name: string;
    language: string;
    description: string;
    unit_price: number;
    unit_time: string;
    discount: SubscribeDiscount[];
    replacement: number;
    inventory: number;
    traffic: number;
    speed_limit: number;
    device_limit: number;
    quota: number;
    nodes: number[];
    node_tags: string[];
    show: boolean;
    sell: boolean;
    sort: number;
    deduction_ratio: number;
    allow_deduction: boolean;
    reset_cycle: number;
    renewal_reset: boolean;
  };

  type UpdateTicketStatusRequest = {
    id: number;
    status: number;
  };

  type UpdateUserAuthMethodRequest = {
    user_id: number;
    auth_type: string;
    auth_identifier: string;
  };

  type UpdateUserBasiceInfoRequest = {
    user_id: number;
    password: string;
    avatar: string;
    balance: number;
    commission: number;
    referral_percentage: number;
    only_first_purchase: boolean;
    gift_amount: number;
    telegram: number;
    refer_code: string;
    referer_id: number;
    enable: boolean;
    is_admin: boolean;
  };

  type UpdateUserNotifySettingRequest = {
    user_id: number;
    enable_balance_notify: boolean;
    enable_login_notify: boolean;
    enable_subscribe_notify: boolean;
    enable_trade_notify: boolean;
  };

  type UpdateUserSubscribeRequest = {
    user_subscribe_id: number;
    subscribe_id: number;
    traffic: number;
    expired_at: number;
    upload: number;
    download: number;
  };

  type User = {
    id: number;
    avatar: string;
    balance: number;
    commission: number;
    referral_percentage: number;
    only_first_purchase: boolean;
    gift_amount: number;
    telegram: number;
    refer_code: string;
    referer_id: number;
    enable: boolean;
    is_admin?: boolean;
    enable_balance_notify: boolean;
    enable_login_notify: boolean;
    enable_subscribe_notify: boolean;
    enable_trade_notify: boolean;
    auth_methods: UserAuthMethod[];
    user_devices: UserDevice[];
    created_at: number;
    updated_at: number;
    deleted_at?: number;
    is_del?: boolean;
  };

  type UserAffiliate = {
    avatar: string;
    identifier: string;
    registered_at: number;
    enable: boolean;
  };

  type UserAuthMethod = {
    auth_type: string;
    auth_identifier: string;
    verified: boolean;
  };

  type UserDevice = {
    id: number;
    ip: string;
    identifier: string;
    user_agent: string;
    online: boolean;
    enabled: boolean;
    created_at: number;
    updated_at: number;
  };

  type UserLoginLog = {
    id: number;
    user_id: number;
    login_ip: string;
    user_agent: string;
    success: boolean;
    timestamp: number;
  };

  type UserStatistics = {
    date?: string;
    register: number;
    new_order_users: number;
    renewal_order_users: number;
    list?: UserStatistics[];
  };

  type UserStatisticsResponse = {
    today: UserStatistics;
    monthly: UserStatistics;
    all: UserStatistics;
  };

  type UserSubscribe = {
    id: number;
    user_id: number;
    order_id: number;
    subscribe_id: number;
    subscribe: Subscribe;
    start_time: number;
    expire_time: number;
    finished_at: number;
    reset_time: number;
    traffic: number;
    download: number;
    upload: number;
    token: string;
    status: number;
    note: string;
    created_at: number;
    updated_at: number;
  };

  type UserSubscribeDetail = {
    id: number;
    user_id: number;
    user: User;
    order_id: number;
    subscribe_id: number;
    subscribe: Subscribe;
    start_time: number;
    expire_time: number;
    reset_time: number;
    traffic: number;
    download: number;
    upload: number;
    token: string;
    status: number;
    note: string;
    created_at: number;
    updated_at: number;
  };

  type UserSubscribeLog = {
    id: number;
    user_id: number;
    user_subscribe_id: number;
    token: string;
    ip: string;
    user_agent: string;
    timestamp: number;
  };

  type UserSubscribeTrafficLog = {
    /** Subscribe ID */
    subscribe_id: number;
    /** User ID */
    user_id: number;
    /** Upload traffic in bytes */
    upload: number;
    /** Download traffic in bytes */
    download: number;
    /** Total traffic in bytes (Upload + Download) */
    total: number;
    /** Date in YYYY-MM-DD format */
    date: string;
    /** Whether to show detailed traffic */
    details: boolean;
  };

  type UserTrafficData = {
    sid: number;
    upload: number;
    download: number;
  };

  type VerifyCodeConfig = {
    verify_code_expire_time: number;
    verify_code_limit: number;
    verify_code_interval: number;
  };

  type VerifyConfig = {
    turnstile_site_key: string;
    turnstile_secret: string;
    enable_login_verify: boolean;
    enable_register_verify: boolean;
    enable_reset_password_verify: boolean;
  };

  type VersionResponse = {
    version: string;
  };

  type Vless = {
    port: number;
    flow: string;
    transport: string;
    transport_config: TransportConfig;
    security: string;
    security_config: SecurityConfig;
  };

  type Vmess = {
    port: number;
    transport: string;
    transport_config: TransportConfig;
    security: string;
    security_config: SecurityConfig;
  };
}
