'use client';

import { Display } from '@/components/display';
import EditNote from '@/components/subscribe/edit-note';
import Renewal from '@/components/subscribe/renewal';
import ResetTraffic from '@/components/subscribe/reset-traffic';
import Unsubscribe from '@/components/subscribe/unsubscribe';
import useGlobalStore from '@/config/use-global';
import { getClient, getStat } from '@/services/common/common';
import { queryUserSubscribe, resetUserSubscribeToken } from '@/services/user/user';
import { getPlatform } from '@/utils/common';
import { useQuery } from '@tanstack/react-query';
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from '@workspace/ui/components/accordion';
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from '@workspace/ui/components/alert-dialog';
import { Button } from '@workspace/ui/components/button';
import { Card, CardContent, CardHeader, CardTitle } from '@workspace/ui/components/card';
import { Separator } from '@workspace/ui/components/separator';
import { Tabs, TabsList, TabsTrigger } from '@workspace/ui/components/tabs';
import { Icon } from '@workspace/ui/custom-components/icon';
import { cn } from '@workspace/ui/lib/utils';
import { differenceInDays, formatDate, isBrowser } from '@workspace/ui/utils';
import { useTranslations } from 'next-intl';
import Image from 'next/image';
import Link from 'next/link';
import { QRCodeCanvas } from 'qrcode.react';
import React, { useState } from 'react';
import CopyToClipboard from 'react-copy-to-clipboard';
import { toast } from 'sonner';
import Subscribe from '../subscribe/page';

const platforms: (keyof API.DownloadLink)[] = [
  'windows',
  'mac',
  'linux',
  'ios',
  'android',
  'harmony',
];

export default function Content() {
  const t = useTranslations('dashboard');
  const { getUserSubscribe, getAppSubLink } = useGlobalStore();

  const [protocol, setProtocol] = useState('');

  const {
    data: userSubscribe = [],
    refetch,
    isLoading,
  } = useQuery({
    queryKey: ['queryUserSubscribe'],
    queryFn: async () => {
      const { data } = await queryUserSubscribe();
      return data.data?.list || [];
    },
  });
  const { data: applications } = useQuery({
    queryKey: ['getClient'],
    queryFn: async () => {
      const { data } = await getClient();
      return data.data?.list || [];
    },
  });

  const availablePlatforms = React.useMemo(() => {
    if (!applications || applications.length === 0) return platforms;

    const platformsSet = new Set<keyof API.DownloadLink>();

    applications.forEach((app) => {
      if (app.download_link) {
        platforms.forEach((platform) => {
          if (app.download_link?.[platform]) {
            platformsSet.add(platform);
          }
        });
      }
    });

    return platforms.filter((platform) => platformsSet.has(platform));
  }, [applications]);

  const [platform, setPlatform] = useState<keyof API.DownloadLink>(() => {
    const detectedPlatform =
      getPlatform() === 'macos' ? 'mac' : (getPlatform() as keyof API.DownloadLink);
    return detectedPlatform;
  });

  React.useEffect(() => {
    if (availablePlatforms.length > 0 && !availablePlatforms.includes(platform)) {
      const firstAvailablePlatform = availablePlatforms[0];
      if (firstAvailablePlatform) {
        setPlatform(firstAvailablePlatform);
      }
    }
  }, [availablePlatforms, platform]);

  const { data } = useQuery({
    queryKey: ['getStat'],
    queryFn: async () => {
      const { data } = await getStat({
        skipErrorHandler: true,
      });
      return data.data;
    },
    refetchOnWindowFocus: false,
  });

  const statusWatermarks = {
    2: t('finished'),
    3: t('expired'),
    4: t('deducted'),
  };

  return (
    <>
      {userSubscribe.length ? (
        <>
          <div className='flex items-center justify-between'>
            <h2 className='flex items-center gap-1.5 font-semibold'>
              <Icon icon='uil:servers' className='size-5' />
              {t('mySubscriptions')}
            </h2>
            <div className='flex gap-2'>
              <Button
                size='sm'
                variant='outline'
                onClick={() => {
                  refetch();
                }}
                className={isLoading ? 'animate-pulse' : ''}
              >
                <Icon icon='uil:sync' />
              </Button>
              <Button size='sm' asChild>
                <Link href='/subscribe'>{t('purchaseSubscription')}</Link>
              </Button>
            </div>
          </div>
          <div className='flex flex-wrap justify-between gap-4'>
            {availablePlatforms.length > 0 && (
              <Tabs
                value={platform}
                onValueChange={(value) => setPlatform(value as keyof API.DownloadLink)}
                className='w-full max-w-full md:w-auto'
              >
                <TabsList className='flex *:flex-auto'>
                  {availablePlatforms.map((item) => (
                    <TabsTrigger value={item} key={item} className='px-1 lg:px-3'>
                      <Icon
                        icon={`${
                          {
                            windows: 'mdi:microsoft-windows',
                            mac: 'uil:apple',
                            linux: 'uil:linux',
                            ios: 'simple-icons:ios',
                            android: 'uil:android',
                            harmony: 'simple-icons:harmonyos',
                          }[item]
                        }`}
                        className='size-5'
                      />
                    </TabsTrigger>
                  ))}
                </TabsList>
              </Tabs>
            )}
            {data?.protocol && data?.protocol.length > 1 && (
              <Tabs
                value={protocol}
                onValueChange={setProtocol}
                className='w-full max-w-full md:w-auto'
              >
                <TabsList className='flex *:flex-auto'>
                  {['all', ...(data?.protocol || [])].map((item) => (
                    <TabsTrigger
                      value={item === 'all' ? '' : item}
                      key={item}
                      className='px-1 uppercase lg:px-3'
                    >
                      {item}
                    </TabsTrigger>
                  ))}
                </TabsList>
              </Tabs>
            )}
          </div>
          {userSubscribe.map((item) => {
            return (
              <Card
                key={item.id}
                className={cn('relative', {
                  'relative opacity-80 grayscale': item.status === 3,
                  'relative hidden opacity-60 blur-[0.3px] grayscale': item.status === 4,
                })}
              >
                {item.status >= 2 && (
                  <div
                    className={cn(
                      'pointer-events-none absolute left-0 top-0 z-10 h-full w-full overflow-hidden mix-blend-difference',
                      {
                        'text-destructive': item.status === 2,
                        'text-white': item.status === 3 || item.status === 4,
                      },
                    )}
                    style={{
                      filter: 'contrast(200%) brightness(150%) invert(0.2)',
                    }}
                  >
                    <div className='absolute inset-0'>
                      {Array.from({ length: 16 }).map((_, i) => {
                        const row = Math.floor(i / 4);
                        const col = i % 4;
                        const top = 10 + row * 25 + (col % 2 === 0 ? 5 : -5);
                        const left = 5 + col * 30 + (row % 2 === 0 ? 0 : 10);

                        return (
                          <span
                            key={i}
                            className='absolute rotate-[-30deg] whitespace-nowrap text-lg font-black opacity-40'
                            style={{
                              top: `${top}%`,
                              left: `${left}%`,
                              textShadow: '0px 0px 1px rgba(255,255,255,0.5)',
                            }}
                          >
                            {statusWatermarks[item.status as keyof typeof statusWatermarks]}
                          </span>
                        );
                      })}
                    </div>
                  </div>
                )}
                <CardHeader className='flex flex-row flex-wrap items-center justify-between gap-2 space-y-0'>
                  <CardTitle className='font-medium'>
                    {item.subscribe.name}
                    <p className='text-foreground/50 mt-1 text-sm'>{formatDate(item.start_time)}</p>
                    {item.note && (
                      <p className='text-muted-foreground mt-2 flex items-center gap-1.5 text-sm font-normal'>
                        <Icon icon='uil:file-edit-alt' className='size-4' />
                        {item.note}
                      </p>
                    )}
                  </CardTitle>
                  {item.status !== 4 && (
                    <div className='flex flex-wrap gap-2'>
                      <EditNote id={item.id} currentNote={item.note} onSuccess={refetch} />
                      <AlertDialog>
                        <AlertDialogTrigger asChild>
                          <Button size='sm' variant='destructive'>
                            {t('resetSubscription')}
                          </Button>
                        </AlertDialogTrigger>
                        <AlertDialogContent>
                          <AlertDialogHeader>
                            <AlertDialogTitle>{t('prompt')}</AlertDialogTitle>
                            <AlertDialogDescription>
                              {t('confirmResetSubscription')}
                            </AlertDialogDescription>
                          </AlertDialogHeader>
                          <AlertDialogFooter>
                            <AlertDialogCancel>{t('cancel')}</AlertDialogCancel>
                            <AlertDialogAction
                              onClick={async () => {
                                await resetUserSubscribeToken({
                                  user_subscribe_id: item.id,
                                });
                                await refetch();
                                toast.success(t('resetSuccess'));
                              }}
                            >
                              {t('confirm')}
                            </AlertDialogAction>
                          </AlertDialogFooter>
                        </AlertDialogContent>
                      </AlertDialog>
                      <ResetTraffic id={item.id} replacement={item.subscribe.replacement} />
                      <Renewal id={item.id} subscribe={item.subscribe} />

                      <Unsubscribe id={item.id} allowDeduction={item.subscribe.allow_deduction} />
                    </div>
                  )}
                </CardHeader>
                <CardContent>
                  <ul className='grid grid-cols-2 gap-3 *:flex *:flex-col *:justify-between lg:grid-cols-4'>
                    <li>
                      <span className='text-muted-foreground'>{t('used')}</span>
                      <span className='text-2xl font-bold'>
                        <Display
                          type='traffic'
                          value={item.upload + item.download}
                          unlimited={!item.traffic}
                        />
                      </span>
                    </li>
                    <li>
                      <span className='text-muted-foreground'>{t('totalTraffic')}</span>
                      <span className='text-2xl font-bold'>
                        <Display type='traffic' value={item.traffic} unlimited={!item.traffic} />
                      </span>
                    </li>
                    <li>
                      <span className='text-muted-foreground'>{t('nextResetDays')}</span>
                      <span className='text-2xl font-semibold'>
                        {item.reset_time
                          ? differenceInDays(new Date(item.reset_time), new Date())
                          : t('noReset')}
                      </span>
                    </li>
                    <li>
                      <span className='text-muted-foreground'>{t('expirationDays')}</span>
                      <span className='text-2xl font-semibold'>
                        {}
                        {item.expire_time
                          ? differenceInDays(new Date(item.expire_time), new Date()) || t('unknown')
                          : t('noLimit')}
                      </span>
                    </li>
                  </ul>
                  <Separator className='mt-4' />
                  <Accordion type='single' collapsible defaultValue='0' className='w-full'>
                    {getUserSubscribe(item.token, protocol)?.map((url, index) => (
                      <AccordionItem key={url} value={String(index)}>
                        <AccordionTrigger className='hover:no-underline'>
                          <div className='flex w-full flex-row items-center justify-between'>
                            <CardTitle className='text-sm font-medium'>
                              {t('subscriptionUrl')} {index + 1}
                            </CardTitle>

                            <CopyToClipboard
                              text={url}
                              onCopy={(text, result) => {
                                if (result) {
                                  toast.success(t('copySuccess'));
                                }
                              }}
                            >
                              <span
                                className='text-primary hover:bg-accent mr-4 flex cursor-pointer rounded p-2 text-sm'
                                onClick={(e) => e.stopPropagation()}
                              >
                                <Icon icon='uil:copy' className='mr-2 size-5' />
                                {t('copy')}
                              </span>
                            </CopyToClipboard>
                          </div>
                        </AccordionTrigger>
                        <AccordionContent>
                          <div className='grid grid-cols-2 gap-4 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-6'>
                            {applications
                              ?.filter((application) => {
                                return !!(
                                  application.download_link?.[platform] && application.scheme
                                );
                              })
                              .map((application) => {
                                const downloadUrl = application.download_link?.[platform];

                                const handleCopy = (text: string, result: boolean) => {
                                  if (result) {
                                    const href = getAppSubLink(url, application.scheme);
                                    const showSuccessMessage = () => {
                                      toast.success(
                                        <>
                                          <p>{t('copySuccess')}</p>
                                          <br />
                                          <p>{t('manualImportMessage')}</p>
                                        </>,
                                      );
                                    };

                                    if (isBrowser() && href) {
                                      window.location.href = href;
                                      const checkRedirect = setTimeout(() => {
                                        if (window.location.href !== href) {
                                          showSuccessMessage();
                                        }
                                        clearTimeout(checkRedirect);
                                      }, 1000);
                                      return;
                                    }

                                    showSuccessMessage();
                                  }
                                };

                                return (
                                  <div
                                    key={application.name}
                                    className='text-muted-foreground flex size-full flex-col items-center justify-between gap-2 text-xs'
                                  >
                                    <span>{application.name}</span>

                                    {application.icon && (
                                      <Image
                                        src={application.icon}
                                        alt={application.name}
                                        width={64}
                                        height={64}
                                        className='p-1'
                                      />
                                    )}
                                    <div className='flex'>
                                      {downloadUrl && (
                                        <Button
                                          size='sm'
                                          variant='secondary'
                                          className={
                                            application.scheme ? 'rounded-r-none px-1.5' : 'px-1.5'
                                          }
                                          asChild
                                        >
                                          <Link href={downloadUrl}>{t('download')}</Link>
                                        </Button>
                                      )}

                                      {application.scheme && (
                                        <CopyToClipboard
                                          text={getAppSubLink(url, application.scheme)}
                                          onCopy={handleCopy}
                                        >
                                          <Button
                                            size='sm'
                                            className={downloadUrl ? 'rounded-l-none p-2' : 'p-2'}
                                          >
                                            {t('import')}
                                          </Button>
                                        </CopyToClipboard>
                                      )}
                                    </div>
                                  </div>
                                );
                              })}
                            <div className='text-muted-foreground hidden size-full flex-col items-center justify-between gap-2 text-sm lg:flex'>
                              <span>{t('qrCode')}</span>
                              <QRCodeCanvas
                                value={url}
                                size={80}
                                bgColor='transparent'
                                fgColor='rgb(59, 130, 246)'
                              />
                              <span className='text-center'>{t('scanToSubscribe')}</span>
                            </div>
                          </div>
                        </AccordionContent>
                      </AccordionItem>
                    ))}
                  </Accordion>
                </CardContent>
              </Card>
            );
          })}
        </>
      ) : (
        <>
          <h2 className='flex items-center gap-1.5 font-semibold'>
            <Icon icon='uil:shop' className='size-5' />
            {t('purchaseSubscription')}
          </h2>
          <Subscribe />
        </>
      )}
    </>
  );
}
