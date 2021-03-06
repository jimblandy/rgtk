// This file is part of rgtk.
//
// rgtk is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// rgtk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with rgtk.  If not, see <http://www.gnu.org/licenses/>.

//! GtkActionable — An interface for widgets that can be associated with actions

use std::ffi::CString;
use gtk::cast::GTK_ACTIONABLE;
use gtk::{self, ffi};

pub trait ActionableTrait: gtk::WidgetTrait {
    fn get_action_name(&self) -> Option<String> {
        unsafe {
            let tmp_pointer = ffi::gtk_actionable_get_action_name(GTK_ACTIONABLE(self.unwrap_widget()));

            if tmp_pointer.is_null() {
                None
            } else {
                Some(String::from_utf8_lossy(::std::ffi::c_str_to_bytes(&tmp_pointer)).to_string())
            }
        }
    }

    fn set_action_name(&self, action_name: &str) {
        unsafe {
            let c_str = CString::from_slice(action_name.as_bytes());

            ffi::gtk_actionable_set_action_name(GTK_ACTIONABLE(self.unwrap_widget()), c_str.as_ptr())
        }
    }

    fn set_detailed_action_name(&self, detailed_action_name: &str) {
        unsafe {
            let c_str = CString::from_slice(detailed_action_name.as_bytes());

            ffi::gtk_actionable_set_detailed_action_name(GTK_ACTIONABLE(self.unwrap_widget()), c_str.as_ptr())
        }
    }
}
