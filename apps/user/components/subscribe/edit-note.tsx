'use client';

import { updateUserSubscribeNote } from '@/services/user/subscribe';
import { Button } from '@workspace/ui/components/button';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@workspace/ui/components/dialog';
import { Textarea } from '@workspace/ui/components/textarea';
import { Icon } from '@workspace/ui/custom-components/icon';
import { LoaderCircle } from 'lucide-react';
import { useTranslations } from 'next-intl';
import { useCallback, useState, useTransition } from 'react';
import { toast } from 'sonner';

interface EditNoteProps {
  id: number;
  currentNote?: string;
  onSuccess?: () => void;
}

export default function EditNote({ id, currentNote = '', onSuccess }: Readonly<EditNoteProps>) {
  const t = useTranslations('dashboard');
  const [open, setOpen] = useState<boolean>(false);
  const [note, setNote] = useState<string>(currentNote);
  const [loading, startTransition] = useTransition();

  const handleSubmit = useCallback(async () => {
    startTransition(async () => {
      try {
        await updateUserSubscribeNote({
          user_subscribe_id: id,
          note: note,
        });
        toast.success(t('noteUpdateSuccess'));
        setOpen(false);
        if (onSuccess) {
          onSuccess();
        }
      } catch (error) {
        toast.error(t('noteUpdateFailed'));
      }
    });
  }, [id, note, onSuccess, t]);

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger asChild>
        <Button size='sm' variant='outline'>
          <Icon icon='uil:edit' className='mr-1 size-4' />
          {t('editNote')}
        </Button>
      </DialogTrigger>
      <DialogContent className='sm:max-w-[425px]'>
        <DialogHeader>
          <DialogTitle>{t('editNote')}</DialogTitle>
          <DialogDescription>{t('noteDescription')}</DialogDescription>
        </DialogHeader>
        <div className='grid gap-4 py-4'>
          <Textarea
            placeholder={t('notePlaceholder')}
            value={note}
            onChange={(e) => setNote(e.target.value)}
            maxLength={500}
            rows={4}
            className='resize-none'
          />
          <p className='text-muted-foreground text-xs'>
            {note.length}/500 {t('characters')}
          </p>
        </div>
        <DialogFooter>
          <Button variant='outline' onClick={() => setOpen(false)} disabled={loading}>
            {t('cancel')}
          </Button>
          <Button onClick={handleSubmit} disabled={loading}>
            {loading && <LoaderCircle className='mr-2 size-4 animate-spin' />}
            {t('save')}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
