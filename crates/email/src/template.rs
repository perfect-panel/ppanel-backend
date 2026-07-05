pub const DEFAULT_EMAIL_VERIFY_TEMPLATE: &str = r#"<!doctype html>
<html>
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>
      {{if eq .Type 1}}注册验证码 / Registration Verification Code{{else}}重置密码验证码 / Password
      Reset Verification Code{{end}}
    </title>
    <style>
      body { color: black; }
      .container { border-radius: 5px; width: 500px; margin: 20px auto 0; border: 1px solid #cce7ff; background-color: #f0f8ff; padding: 25px 30px; }
      .header { text-align: center; display: flex; align-items: center; justify-content: center; }
      .logo { width: 56px; height: 56px; object-fit: cover; margin-right: 10px; }
      .site-name { font-size: 18px; font-weight: bold; margin: 0; }
      .content { margin: 10px 0; font-size: 14px; }
      .greeting { font-weight: 700; margin: 5px 0; }
      .highlight { margin: 0 2px; font-weight: 700; color: #007bff; }
      .code-container { margin: 25px 0; width: 100%; background-color: #e6f2ff; height: 60px; line-height: 60px; text-align: center; font-size: 32px; font-weight: 700; color: #007bff; }
      .code { letter-spacing: 5pt; }
      .footer { border-top: #99ccff 1px solid; margin-top: 20px; padding-top: 5px; font-size: 12px; font-weight: 700; color: #777; }
    </style>
  </head>
  <body>
    <div class="container">
      <div class="header">
        <img src="{{.SiteLogo}}" class="logo" />
        <p class="site-name">{{.SiteName}}</p>
      </div>
      <div class="content">
        <p class="greeting">Hi, 尊敬的用户 / Dear User</p>
        <p>
          {{if eq .Type 1}} 感谢您注册！您的验证码是（请于<span class="highlight">{{.Expire}}</span>分钟内使用）：
          <br />
          Thank you for registering! Your verification code is (please use it within <span class="highlight">{{.Expire}}</span> minutes): {{else}}
          您正在重置密码。您的验证码是（请于<span class="highlight">{{.Expire}}</span>分钟内使用）：
          <br />
          You are resetting your password. Your verification code is (please use it within <span class="highlight">{{.Expire}}</span> minutes): {{end}}
        </p>
        <div class="code-container">
          <span class="code">{{.Code}}</span>
        </div>
        <p>
          如果您未请求此验证码，请忽略此邮件。<br />If you did not request this code, please ignore this email.
        </p>
      </div>
      <div class="footer">此为系统邮件，请勿回复 / This is a system email, please do not reply</div>
    </div>
  </body>
</html>"#;

pub const DEFAULT_MAINTENANCE_EMAIL_TEMPLATE: &str = r#"<!doctype html>
<html lang="zh-CN">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>系统维护通知 / System Maintenance Notice</title>
    <style>
      body { color: black; }
      .container { border-radius: 5px; width: 500px; margin: 20px auto 0; border: 1px solid #cce7ff; background-color: #f0f8ff; padding: 25px 30px; }
      .header { text-align: center; display: flex; align-items: center; justify-content: center; }
      .logo { width: 56px; height: 56px; object-fit: cover; margin-right: 10px; }
      .site-name { font-size: 18px; font-weight: bold; margin: 0; }
      .content { margin: 20px 0; font-size: 14px; }
      .greeting { font-weight: 700; margin: 5px 0; }
      .highlight { margin: 0 2px; font-weight: 700; color: #007bff; }
      .footer { border-top: #99ccff 1px solid; margin-top: 20px; padding-top: 5px; font-size: 12px; font-weight: 700; color: #777; }
    </style>
  </head>
  <body>
    <div class="container">
      <div class="header">
        <img src="{{.SiteLogo}}" class="logo" />
        <p class="site-name">{{.SiteName}}</p>
      </div>
      <div class="content">
        <p class="greeting">Hi, 尊敬的用户 / Dear User</p>
        <p>
          我们计划在<span class="highlight">{{.MaintenanceDate}}</span>进行系统维护，预计维护时间为<span class="highlight">{{.MaintenanceTime}}</span>。在此期间，您可能会遇到服务中断或无法访问的情况。
          <br />
          We will be performing system maintenance on <span class="highlight">{{.MaintenanceDate}}</span>, and the expected maintenance period is <span class="highlight">{{.MaintenanceTime}}</span>. During this time, you may experience service interruptions or unavailability.
        </p>
        <p>
          维护完成后，系统将自动恢复。如果您有任何问题，请随时联系我们的支持团队。
          <br />
          The system will resume automatically once the maintenance is completed. If you have any questions, please feel free to contact our support team.
        </p>
      </div>
      <div class="footer">此为系统邮件，请勿回复 / This is a system email, please do not reply</div>
    </div>
  </body>
</html>"#;

pub const DEFAULT_EXPIRATION_EMAIL_TEMPLATE: &str = r#"<!doctype html>
<html>
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>服务到期通知 / Service Expiration Notice</title>
    <style>
      body { color: black; }
      .container { border-radius: 5px; width: 500px; margin: 20px auto 0; border: 1px solid #cce7ff; background-color: #f0f8ff; padding: 25px 30px; }
      .header { text-align: center; display: flex; align-items: center; justify-content: center; }
      .logo { width: 56px; height: 56px; object-fit: cover; margin-right: 10px; }
      .site-name { font-size: 18px; font-weight: bold; margin: 0; }
      .content { margin: 20px 0; font-size: 14px; }
      .greeting { font-weight: 700; margin: 5px 0; }
      .highlight { margin: 0 2px; font-weight: 700; color: #007bff; }
      .footer { border-top: #99ccff 1px solid; margin-top: 20px; padding-top: 5px; font-size: 12px; font-weight: 700; color: #777; }
    </style>
  </head>
  <body>
    <div class="container">
      <div class="header">
        <img src="{{.SiteLogo}}" class="logo" />
        <p class="site-name">{{.SiteName}}</p>
      </div>
      <div class="content">
        <p class="greeting">Hi, 尊敬的用户 / Dear User</p>
        <p>
          您的服务即将在<span class="highlight">{{.ExpireDate}}</span>到期，请及时续费以保证服务不间断。
          <br />
          Your service is set to expire on <span class="highlight">{{.ExpireDate}}</span>. Please renew your subscription to avoid service interruptions.
        </p>
        <p>
          如需帮助，请联系客服团队。感谢您的支持！
          <br />
          If you need assistance, please contact our support team. Thank you for your continued support!
        </p>
      </div>
      <div class="footer">此为系统邮件，请勿回复 / This is a system email, please do not reply</div>
    </div>
  </body>
</html>"#;

pub const DEFAULT_TRAFFIC_EXCEED_EMAIL_TEMPLATE: &str = r#"<!doctype html>
<html>
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>流量用尽通知 / Traffic Exhausted Notice</title>
    <style>
      .container { border-radius: 5px; width: 500px; margin: 20px auto 0; border: 1px solid #cce7ff; background-color: #f0f8ff; padding: 25px 30px; }
      .header { text-align: center; display: flex; align-items: center; justify-content: center; }
      .logo { width: 56px; height: 56px; object-fit: cover; margin-right: 10px; }
      .site-name { font-size: 18px; font-weight: bold; margin: 0; }
      .content { margin: 20px 0; font-size: 14px; }
      .greeting { font-weight: 700; margin: 5px 0; }
      .highlight { color: #007bff; }
      .footer { border-top: #99ccff 1px solid; margin-top: 20px; padding-top: 5px; font-size: 12px; font-weight: 700; color: #777; }
    </style>
  </head>
  <body>
    <div class="container">
      <div class="header">
        <img src="{{.SiteLogo}}" class="logo" />
        <p class="site-name">{{.SiteName}}</p>
      </div>
      <div class="content">
        <p class="greeting">Hi, 尊敬的用户 / Dear User</p>
        <p>
          您的流量已经用尽，请及时购买流量以继续使用我们的服务。
          <br />
          Your traffic has been exhausted. Please purchase additional traffic to continue using our service.
        </p>
        <p>
          如需帮助，请联系客服团队。感谢您的支持！
          <br />
          If you need assistance, please contact our support team. Thank you for your continued support!
        </p>
      </div>
      <div class="footer">此为系统邮件，请勿回复 / This is a system email, please do not reply</div>
    </div>
  </body>
</html>"#;
